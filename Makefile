build-frontend:
	wasm-pack build --release --target web frontend

build-frontend-dev:
	wasm-pack build --dev --target web frontend


build-backend:
	cargo build --release -p swikn-backend

build-backend-dev:
	cargo build -p swikn-backend

build: build-frontend build-backend
	
build-dev: build-frontend-dev build-backend-dev

dist: 
	rm -rf dist
	mkdir dist
	mkdir -p dist/static/
	cp -a backend/static/ dist/
	cp frontend/pkg/swikn* dist/static/


run: build dist
	cd dist; cargo run -p swikn-backend

run-dev: build-dev dist
	cd dist; cargo run -p swikn-backend

watch:
	find . | grep -E "\.rs$$|\.html$$" | grep -v -E "^\./dist" | entr -r -s "make run-dev"
