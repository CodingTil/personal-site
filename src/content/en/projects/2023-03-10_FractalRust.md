---
slug: fractal
image: <img loading="lazy" src="images/projects/Fractal/file.webp" alt="Fractal"/>
title: Fractal (Rust + WebAssembly + WGPU)
color: bg-orange-600
tagline: Fractal generator written in Rust and compiled to WebAssembly
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

# Overview
<iframe src="http://192.168.178.33:8080/public/project_code/fractal/index.html" title="Fractal" class="w-full p-2.5 pointer-events-none" style="aspect-ratio: 16 / 9"></iframe>
<i>If nothing is being displayed, it is possible that your browser does not support WebAssembly or GPU rendering has been disabled.</i>

This small program is written in [Rust](https://www.rust-lang.org/), compiled to [WebAssembly](https://webassembly.org/), and uses the GPU ([wgpu](https://wgpu.rs/)) to render the fractal.

I worked on this project because I wanted to explore the capabilities of rendering with the GPU and learn more about Rust programming language. The project aimed to generate fractals using Rust programming language and compile the code into WebAssembly format for easy accessibility on the web.

I utilized the wgpu library, which is a Rust implementation of the WebGPU standard, to interface with the graphics card and render the fractal. Additionally, I experimented with shader programming to enhance the visual appeal of the fractal generated.

Overall, this project allowed me to dive deeper into the world of Rust programming and explore the power of GPU computing. It also provided an opportunity to learn more about web development and build an interactive application accessible to a wider audience.