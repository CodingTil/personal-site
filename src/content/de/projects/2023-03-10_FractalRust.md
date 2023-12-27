---
slug: fractal
image: <img loading="lazy" src="images/projects/Fractal/file.webp" alt="Fractal"/>
title: Fractal (Rust + WebAssembly + WGPU)
color: bg-orange-600
tagline: Fraktalgenerator geschrieben in Rust und kompiliert zu WebAssembly
url: https://github.com/CodingTil/fractal_rust
date_range: March 2023
skills: [rust, webassembly, gpu, shader]
filters: [rust, webassembly, gpu, shader, wasm]
---
<div class="flex items-center justify-center">
	<a href="https://www.rust-lang.org/">
		<i class="text-foreground-primary fa-brands fa-rust" style="font-size: 60px;"></i>
	</a>
	<span class="mx-2 text-2xl">+</span>
	<a href="https://www.rust-lang.org/what/wasm">
		<img width="68px" height="60px" alt="WebAssembly Logo" src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/68px-WebAssembly_Logo.svg.png?useskin=vector">
	</a>
	<span class="mx-2 text-2xl">+</span>
	<a href="https://wgpu.rs/">
		<img src="https://wgpu.rs/logo.min.svg" width="60px" alt="WebGPU Logo">
	</a>
</div>

# Übersicht
<iframe src="/public/project_code/fractal_rust/index.html" title="Fractal" class="w-full p-2.5 pointer-events-none" style="aspect-ratio: 16 / 9"></iframe>
<i>Wenn nichts angezeigt wird, ist es wahrscheinlich, dass entweder Ihr Browser WebAssembly nicht unterstützt oder das GPU-Rendering deaktiviert ist.</i>

Dieses kleine Programm ist in der Programmiersprache [Rust](https://www.rust-lang.org/) geschrieben, zu [WebAssembly](https://webassembly.org/) kompiliert und nutzt die GPU ([wgpu](https://wgpu.rs/)) zur Darstellung des Fraktals.

Ich habe an diesem Projekt gearbeitet, weil ich die Fähigkeiten der GPU zur Darstellung von Grafiken erkunden und mehr über die Programmiersprache Rust lernen wollte. Das Projekt zielt darauf ab, Fraktale mit der Programmiersprache Rust zu generieren und den Code in das WebAssembly-Format zu kompilieren, um eine einfache Zugänglichkeit im Web zu ermöglichen.

Ich habe die wgpu-Bibliothek genutzt, eine Rust-Implementierung des WebGPU-Standards, um mit der Grafikkarte zu kommunizieren und das Fraktal zu rendern. Zusätzlich habe ich mit Shader-Programmierung experimentiert, um die visuelle Attraktivität des generierten Fraktals zu verbessern.

Insgesamt hat mir dieses Projekt ermöglicht, tiefer in die Welt der Rust-Programmierung einzutauchen und die Kraft der GPU-Computing zu erforschen. Es bot auch eine Gelegenheit, mehr über Webentwicklung zu lernen und eine interaktive Anwendung zu erstellen, die einem breiteren Publikum zugänglich ist.