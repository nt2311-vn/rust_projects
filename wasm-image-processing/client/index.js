import * as wasm from "hello-wasm-pack";

function setup(event) {
	const fileInput = document.getElementById("image-upload");

	fileInput.addEventListener("change", function (event) {
		const file = event.target.files[0];
		const imageUrl = window.URL.createObjectURL(file);

		const image = new Image();
		image.src = imageUrl;

		image.addEventListener("load", (loadEvent) => {
			const canvas = document.getElementById("preview");
			canvas.width = image.naturalWidth;
			canvas.height = image.naturalHeight;

			canvas
				.getContext("2d")
				.drawImage(image, 0, 0, canvas.width, canvas.height);
		});
	});
}

if (document.readyState !== "loading") {
	setup();
} else {
	window.addEventListener("DOMContentLoaded", setup);
}
