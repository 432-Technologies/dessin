import http from "node:http";
import fs from "node:fs";
import path from "node:path";
import { spawn } from "node:child_process";

const PORT = parseInt(process.env.PORT || "3000", 10);
const ROOT = new URL(".", import.meta.url).pathname;
const EXAMPLES_DIR = path.join(ROOT, "examples");
const OUT_DIR = path.join(EXAMPLES_DIR, "out");

function listExamples() {
	return fs
		.readdirSync(EXAMPLES_DIR)
		.filter((f) => f.endsWith(".rs"))
		.map((f) => f.slice(0, -3))
		.sort();
}

function snapshotDir(dir) {
	if (!fs.existsSync(dir)) return new Set();
	const entries = fs.readdirSync(dir);
	return new Set(
		entries.filter((f) => {
			const stat = fs.statSync(path.join(dir, f));
			return stat.isFile();
		}),
	);
}

function findNewFiles(dir, before) {
	const after = snapshotDir(dir);
	return [...after].filter((f) => !before.has(f));
}

function runExample(name, cb) {
	const before = snapshotDir(OUT_DIR);
	const proc = spawn("cargo", ["run", "--example", name], {
		cwd: ROOT,
		env: { ...process.env, NO_ANIMATION: "1", NO_ICED: "1" },
		stdio: ["ignore", "pipe", "pipe"],
	});

	const chunks = [];
	proc.stdout.on("data", (c) => chunks.push(c.toString()));
	proc.stderr.on("data", (c) => chunks.push(c.toString()));

	proc.on("close", (code) => {
		const output = chunks.join("");
		const files = findNewFiles(OUT_DIR, before);
		cb({ code, output, files });
	});

	proc.on("error", (err) => cb({ code: -1, output: err.message, files: [] }));
}

function embedOutput(name, file) {
	const ext = path.extname(file).toLowerCase();
	const rel = `out/${file}`;
	const label = file;

	if (ext === ".svg") {
		const src = fs.readFileSync(path.join(OUT_DIR, file), "utf8");
		return `<figure><figcaption>${name} – ${label}</figcaption>${src}</figure>`;
	}

	if ([".png", ".jpg", ".jpeg", ".gif", ".webp", ".bmp"].includes(ext)) {
		return `<figure><figcaption>${name} – ${label}</figcaption><img src="${rel}" alt="${label}" /></figure>`;
	}

	if (ext === ".pdf") {
		return `<figure><figcaption>${name} – ${label}</figcaption><iframe src="${rel}" title="${label}" style="width:100%;height:600px;border:1px solid #ddd"></iframe><p><a href="${rel}" download>Download ${label}</a></p></figure>`;
	}

	return `<figure><figcaption>${name} – ${label} (unsupported type)</figcaption><p><a href="${rel}" download>Download ${label}</a></p></figure>`;
}

const PAGE = `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Dessin Visualizer</title>
<style>
  *, *::before, *::after { box-sizing: border-box; }
  body { margin: 0; font-family: system-ui, -apple-system, sans-serif; background: #f8f9fa; color: #212529; }
  header { background: #212529; color: #fff; padding: 16px 24px; display: flex; align-items: center; gap: 16px; }
  header h1 { font-size: 1.25rem; margin: 0; font-weight: 600; }
  select { padding: 8px 12px; font-size: 0.95rem; border-radius: 6px; border: none; cursor: pointer; min-width: 240px; }
  #status { margin-left: auto; font-size: 0.85rem; opacity: 0.8; min-width: 120px; text-align: right; }
  main { max-width: 1100px; margin: 0 auto; padding: 24px; }
  .result { background: #fff; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,.1); margin-bottom: 32px; overflow: hidden; }
  .result-header { padding: 12px 20px; border-bottom: 1px solid #eee; font-weight: 600; font-size: 0.95rem; }
  .result-body { padding: 20px; }
  figure { margin: 0 0 24px; }
  figure:last-child { margin-bottom: 0; }
  figcaption { font-size: 0.8rem; color: #6c757d; margin-bottom: 8px; font-weight: 500; }
  img { max-width: 100%; height: auto; border-radius: 4px; }
  svg { max-width: 100%; height: auto; display: block; }

</style>
</head>
<body>
<header>
  <h1>Dessin Visualizer</h1>
  <select id="examples"><option value="">Select an example…</option></select>
  <span id="status"></span>
</header>
<main id="results"></main>
<script>
const sel = document.getElementById('examples');
const results = document.getElementById('results');
const status = document.getElementById('status');

function updateUrl(name) {
  const url = new URL(window.location);
  url.searchParams.set('example', name);
  history.pushState(null, '', url);
}

async function run(name) {
  if (!name) return;
  status.textContent = 'Running…';
  results.innerHTML = '';
  sel.value = name;
  updateUrl(name);
  const res = await fetch('/api/run/' + encodeURIComponent(name));
  const data = await res.json();
  status.textContent = data.code === 0 ? 'Done ✓' : 'Failed ✗ (code ' + data.code + ')';

  const card = document.createElement('div');
  card.className = 'result';

  const header = document.createElement('div');
  header.className = 'result-header';
  header.textContent = name;
  card.appendChild(header);

  const body = document.createElement('div');
  body.className = 'result-body';

  if (data.files.length > 0) {
    const outputDiv = document.createElement('div');
    outputDiv.innerHTML = data.html;
    body.appendChild(outputDiv);
  } else {
    body.textContent = 'No output files produced.';
  }

  card.appendChild(body);
  results.appendChild(card);
}

sel.addEventListener('change', () => run(sel.value));

window.addEventListener('popstate', () => {
  const name = new URL(window.location).searchParams.get('example');
  if (name) run(name);
});

// auto-run on load if URL has ?example=foo
(function() {
  const name = new URL(window.location).searchParams.get('example');
  if (name) run(name);
})();
</script>
</body>
</html>`;

const server = http.createServer((req, res) => {
	const url = new URL(req.url, `http://localhost:${PORT}`);

	if (url.pathname === "/" || url.pathname === "/index.html") {
		const examples = listExamples();
		const page = PAGE.replace(
			'<option value="">Select an example…</option>',
			examples.map((e) => `<option value="${e}">${e}</option>`).join(""),
		);
		res.writeHead(200, {
			"Content-Type": "text/html; charset=utf-8",
			"Cache-Control": "no-store, no-cache, must-revalidate",
		});
		res.end(page);
		return;
	}

	if (url.pathname.startsWith("/api/run/")) {
		const name = decodeURIComponent(url.pathname.slice("/api/run/".length));
		res.writeHead(200, {
			"Content-Type": "application/json",
			"Cache-Control": "no-store",
		});

		runExample(name, ({ code, output, files }) => {
			res.end(
				JSON.stringify({
					code,
					files,
					html: files.map((f) => embedOutput(name, f)).join(""),
				}),
			);
		});
		return;
	}

	if (url.pathname.startsWith("/out/")) {
		const file = url.pathname.slice("/out/".length);
		const filePath = path.join(OUT_DIR, file);
		if (!fs.existsSync(filePath)) {
			res.writeHead(404);
			res.end("Not found");
			return;
		}
		const ext = path.extname(file).toLowerCase();
		const types = {
			svg: "image/svg+xml",
			png: "image/png",
			jpg: "image/jpeg",
			jpeg: "image/jpeg",
			gif: "image/gif",
			webp: "image/webp",
			pdf: "application/pdf",
		};
		res.writeHead(200, {
			"Content-Type": types[ext.slice(1)] || "application/octet-stream",
		});
		fs.createReadStream(filePath).pipe(res);
		return;
	}

	res.writeHead(404);
	res.end("Not found");
});

server.listen(PORT, "127.0.0.1", () => {
	console.log(`Dessin Visualizer running at http://127.0.0.1:${PORT}`);
});
