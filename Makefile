default: build


build:
	cargo build 
	# cargo build 2>&1 | less -R

run:
	RUST_BACKTRACE=1 ./target/debug/procedural-modeling

start:
	RUST_BACKTRACE=1 ./target/debug/procedural-modeling & echo $$! >.pidfile

stop:
	kill $$(cat .pidfile)

auto:
	while true; do \
		make stop; make build && make start; \
		FILES="$$(find ./src -name '*.rs' -and -not -name '.*')"; \
		echo $$FILES; \
		inotifywait -e move -e delete -e create -e close_write $$FILES; \
	done
