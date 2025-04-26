build:
	# build web app first
	cd soundshouter-vite-webapp && npm install && npm run build
	# build server, which embeds webapp
	cd soundshouter && cargo build --release
