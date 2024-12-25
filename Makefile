build:
	# build web app first
	cd soundshouter-vite-webapp && npm run build
	# build server, which embeds webapp
	cd soundshouter && cargo build --release
