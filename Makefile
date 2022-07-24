default:
	@echo 'Targets:'
	@echo '  tests          -- Run tests.'
	@echo '  tests-verbose  -- Run tests, include detailed info and program output.'

tests:
	cargo test

tests-verbose:
	cargo test -v -- --nocapture
