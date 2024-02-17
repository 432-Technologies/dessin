use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Bracket, Comma, Paren},
    Expr, ExprAssign, ExprForLoop, ExprIndex, ExprParen, ExprStruct, FieldValue, Path, Result,
    Token,
};

mod kw {
    syn::custom_keyword!(var);
    syn::custom_keyword!(cloned);
}

enum Action {
    WithArgs(ExprAssign),
    WithoutArgs(Ident),
}
impl Parse for Action {
    fn parse(input: ParseStream) -> Result<Self> {
        match input.fork().parse::<ExprAssign>() {
            Ok(v) => input.parse().map(Action::WithArgs),
            Err(_) => input.parse().map(Action::WithoutArgs),
        }
    }
}
impl From<Action> for TokenStream {
    fn from(value: Action) -> Self {
        match value {
            Action::WithArgs(ExprAssign {
                attrs,
                left,
                eq_token,
                right,
            }) => quote!(__current_shape__.#left(#right);),
            Action::WithoutArgs(member) => quote!(__current_shape__.#member();),
        }
    }
}

struct Actions(Punctuated<Action, Comma>);
impl Parse for Actions {
    fn parse(input: ParseStream) -> Result<Self> {
        let actions;
        let _ = parenthesized!(actions in input);
        let actions = actions.parse_terminated(Action::parse, Comma)?;

        Ok(Actions(actions))
    }
}
impl From<Actions> for TokenStream {
    fn from(Actions(actions): Actions) -> Self {
        actions
            .into_iter()
            .map(TokenStream::from)
            .collect::<TokenStream>()
    }
}

struct DessinItem {
    item: Path,
    actions: Actions,
}
impl Parse for DessinItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let item = input.parse::<Path>()?;
        let actions = input.parse::<Actions>()?;

        Ok(DessinItem { item, actions })
    }
}
impl From<DessinItem> for TokenStream {
    fn from(DessinItem { item, actions }: DessinItem) -> Self {
        if actions.0.is_empty() {
            return quote!(
                ::dessin::prelude::Style::new(<#item>::default())
            );
        }

        let actions = TokenStream::from(actions);

        quote!({
            let mut __current_shape__ = ::dessin::prelude::Style::new(<#item>::default());
            #actions
            __current_shape__
        })
    }
}

struct DessinVar {
    var: Expr,
    actions: Actions,
}
impl Parse for DessinVar {
    fn parse(input: ParseStream) -> Result<Self> {
        let _ = input.parse::<kw::var>()?;
        let var;
        let _ = bracketed!(var in input);
        let var = var.parse::<Expr>()?;

        let actions = input.parse::<Actions>()?;

        Ok(DessinVar { var, actions })
    }
}
impl From<DessinVar> for TokenStream {
    fn from(DessinVar { var, actions }: DessinVar) -> Self {
        if actions.0.is_empty() {
            quote!(#var)
        } else {
            let actions = TokenStream::from(actions);

            quote!({
                let mut __current_shape__ = #var;
                #actions
                __current_shape__
            })
        }
    }
}

struct DessinCloned {
    var: Expr,
    actions: Actions,
}
impl Parse for DessinCloned {
    fn parse(input: ParseStream) -> Result<Self> {
        let _ = input.parse::<kw::cloned>()?;
        let var;
        let _ = bracketed!(var in input);
        let var = var.parse::<Expr>()?;

        let actions = input.parse::<Actions>()?;

        Ok(DessinCloned { var, actions })
    }
}
impl From<DessinCloned> for TokenStream {
    fn from(DessinCloned { var, actions }: DessinCloned) -> Self {
        if actions.0.is_empty() {
            quote!(#var)
        } else {
            let actions = TokenStream::from(actions);

            quote!({
                let mut __current_shape__ = (#var).clone();
                #actions
                __current_shape__
            })
        }
    }
}

struct DessinFor {
    expr: ExprForLoop,
}
impl Parse for DessinFor {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr = input.parse::<ExprForLoop>()?;

        Ok(DessinFor { expr })
    }
}
impl From<DessinFor> for TokenStream {
    fn from(
        DessinFor {
            expr:
                ExprForLoop {
                    attrs: _,
                    label: _,
                    for_token: _,
                    pat,
                    in_token: _,
                    expr,
                    body,
                },
        }: DessinFor,
    ) -> Self {
        quote!(::dessin::prelude::Shape::Group(::dessin::prelude::Group {
            metadata: ::std::vec::Vec::new(),
            local_transform: ::dessin::nalgebra::Transform2::default(),
            shapes: {
                let __current_iterator__ = (#expr).into_iter();
                let mut __current_shapes__ = ::std::vec::Vec::with_capacity(__current_iterator__.size_hint().0);
                for #pat in __current_iterator__ {
                    let __current_shape__ = ::dessin::prelude::Shape::from(#body);
                    __current_shapes__.push(__current_shape__);
                }
                __current_shapes__
            },
        }))
    }
}

struct DessinIfElse {
    condition: Expr,
    if_body: Box<Dessin>,
    else_body: Option<Box<Dessin>>,
}
impl Parse for DessinIfElse {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Token![if] = input.parse()?;
        let condition = input.parse::<Expr>()?;
        let if_body;
        let _ = braced!(if_body in input);
        let if_body: Dessin = if_body.parse()?;
        let else_body = if input.parse::<Token![else]>().is_ok() {
            let else_body;
            let _ = braced!(else_body in input);
            Some(Box::new(else_body.parse()?))
        } else {
            None
        };

        Ok(DessinIfElse {
            condition,
            if_body: Box::new(if_body),
            else_body,
        })
    }
}
impl From<DessinIfElse> for TokenStream {
    fn from(
        DessinIfElse {
            condition,
            if_body,
            else_body,
        }: DessinIfElse,
    ) -> Self {
        let else_body = if let Some(else_body) = else_body {
            TokenStream::from(*else_body)
        } else {
            TokenStream::from(DessinType::Empty)
        };

        let if_body = TokenStream::from(*if_body);

        quote!(
            if #condition {
                ::dessin::prelude::Shape::from(#if_body)
            } else {
                ::dessin::prelude::Shape::from(#else_body)
            }
        )
    }
}

struct DessinGroup(Punctuated<Dessin, Token![,]>);
impl Parse for DessinGroup {
    fn parse(input: ParseStream) -> Result<Self> {
        let children;
        let _ = bracketed!(children in input);

        let children = children.parse_terminated(Dessin::parse, Token![,])?;

        Ok(DessinGroup(children))
    }
}
impl From<DessinGroup> for TokenStream {
    fn from(DessinGroup(children): DessinGroup) -> Self {
        let children = children
            .into_iter()
            .map(TokenStream::from)
            .collect::<Vec<_>>();

        quote!(::dessin::prelude::Shape::Group(::dessin::prelude::Group {
            local_transform: ::dessin::nalgebra::Transform2::default(),
            metadata: ::std::vec::Vec::new(),
            shapes: ::std::vec![
                #(::dessin::prelude::Shape::from(#children)),*
            ],
        }))
    }
}

enum DessinType {
    Empty,
    Item(DessinItem),
    Var(DessinVar),
    Cloned(DessinCloned),
    Group(DessinGroup),
    For(DessinFor),
    IfElse(DessinIfElse),
}
impl Parse for DessinType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Ok(DessinType::Empty)
        } else if input.peek(kw::var) {
            input.parse().map(DessinType::Var)
        } else if input.peek(kw::cloned) {
            input.parse().map(DessinType::Cloned)
        } else if input.peek(Token![for]) {
            input.parse().map(DessinType::For)
        } else if input.peek(Token![if]) {
            input.parse().map(DessinType::IfElse)
        } else if input.peek(Bracket) {
            input.parse().map(DessinType::Group)
        } else {
            input.parse().map(DessinType::Item)
        }
    }
}
impl From<DessinType> for TokenStream {
    fn from(value: DessinType) -> Self {
        match value {
            DessinType::Empty => quote!(::dessin::prelude::Shape::default()),
            DessinType::Item(i) => i.into(),
            DessinType::Group(g) => g.into(),
            DessinType::Var(v) => v.into(),
            DessinType::Cloned(v) => v.into(),
            DessinType::For(f) => f.into(),
            DessinType::IfElse(i) => i.into(),
        }
    }
}

///
pub struct Dessin {
    dessin_type: DessinType,
    erased_type_shape_actions: Option<Actions>,
}
impl Parse for Dessin {
    fn parse(input: ParseStream) -> Result<Self> {
        let dessin_type: DessinType = input.parse()?;
        let erased_type_shape_actions = if input.peek(Token![>]) {
            let _: Token![>] = input.parse()?;
            let actions = input.parse::<Actions>()?;

            Some(actions)
        } else {
            None
        };

        Ok(Dessin {
            dessin_type,
            erased_type_shape_actions,
        })
    }
}
impl From<Dessin> for TokenStream {
    fn from(value: Dessin) -> Self {
        let base = TokenStream::from(value.dessin_type);
        if let Some(actions) = value.erased_type_shape_actions {
            if !actions.0.is_empty() {
                let actions = TokenStream::from(actions);

                return quote!({
                    let mut __current_shape__ = ::dessin::prelude::Shape::from(#base);
                    #actions
                    __current_shape__
                });
            }
        }

        quote!(#base)
    }
}
