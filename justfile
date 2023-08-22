front-end:
  cd ./client && npm run dev

back-end:
  cd ./server && cargo run

dev: front-end back-end  

build-front-end:
  cd ./client && npm run build

build-back-end:
  cd ./server && cargo build --release

build: build-front-end && build-back-end

