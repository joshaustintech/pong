runserver:
	cd web && \
	python3 -m http.server

debug:
	export EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sGL_ENABLE_GET_PROC_ADDRESS -sWASM=1 -sALLOW_MEMORY_GROWTH=1 -sWASM_MEM_MAX=512MB -sTOTAL_MEMORY=512MB -sABORTING_MALLOC=0 -sASYNCIFY -sFORCE_FILESYSTEM=1 -sASSERTIONS=1 -sERROR_ON_UNDEFINED_SYMBOLS=0 -sEXPORTED_RUNTIME_METHODS=ccallcwrap" && \
	cargo build --target wasm32-unknown-emscripten
	cp target/wasm32-unknown-emscripten/debug/space_arcade.wasm web/pong.wasm
	cp target/wasm32-unknown-emscripten/debug/space-arcade.js web/pong.js

release:
	export EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sGL_ENABLE_GET_PROC_ADDRESS -sWASM=1 -sALLOW_MEMORY_GROWTH=1 -sWASM_MEM_MAX=512MB -sTOTAL_MEMORY=512MB -sABORTING_MALLOC=0 -sASYNCIFY -sFORCE_FILESYSTEM=1 -sASSERTIONS=1 -sERROR_ON_UNDEFINED_SYMBOLS=0 -sEXPORTED_RUNTIME_METHODS=ccallcwrap" && \
	cargo build --release --target wasm32-unknown-emscripten
	cp target/wasm32-unknown-emscripten/release/pong.wasm web/pong.wasm
	cp target/wasm32-unknown-emscripten/release/pong.js web/pong.js