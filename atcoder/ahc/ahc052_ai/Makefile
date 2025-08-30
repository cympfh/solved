evaluate:
	@echo "Running evaluate.sh..."
	@result=$$(./evaluate.sh 2>/dev/null); \
	echo "Evaluation result: $$result"; \
	git add -A .; \
	git commit --allow-empty -m "$$(git log -1 --pretty=format:'%s') - Evaluation: $$result"