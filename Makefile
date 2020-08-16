default:
	@echo 'Targets:'
	@echo '  tests    -- Run tests.'
	@echo '  tests-so -- Run tests, but suppress program output. (Useful for reviewing build warnings.)'

tests:
	@cargo run -- -t

tests-so:
	@cargo run -- -t > /dev/null
