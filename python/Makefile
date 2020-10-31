PROTOS_DIR=../proto
PROTOS=$(wildcard $(PROTOS_DIR)/*.proto)
OUTDIR=greeter/helloworld

TWIRP_GEN=protoc
TWIRP_OUTPUT_OPTS=--python_out=$(OUTDIR) --twirpy_out=$(OUTDIR)

gen-twirp: $(PROTOS)
	$(TWIRP_GEN) -I$(PROTOS_DIR) $(TWIRP_OUTPUT_OPTS) $^

run-server:
	poetry run uvicorn greeter.server:app --port=5000

run-client:
	poetry run python greeter/client.py
