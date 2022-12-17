run:
	docker run --name api-website -it --rm -p 8787:80 api-website

build:
	docker build -f Dockerfile -t api-website .

stop:
	docker kill api-website