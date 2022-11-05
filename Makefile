build:
	docker build -t github-summary:v1.0.0 .

start:
	docker run \
		-itd \
		--rm \
		--name github-summary \
		-p 3000:3000 \
		github-summary:v1.0.0
