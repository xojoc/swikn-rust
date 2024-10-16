build-frontend:
	wasm-pack build --target web frontend

build-backend:
	cargo build -p swikn-backend

build: build-frontend build-backend

dist: build
	rm -rf dist
	mkdir dist
	mkdir -p dist/static/
	cp -a backend/static/ dist/
	cp frontend/pkg/swikn* dist/static/


run: dist
	cd dist; cargo run -p swikn-backend
