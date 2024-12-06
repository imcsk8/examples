# Posadev

.env:
	echo "DATABASE_URL=postgres://postgres:prueba123@127.0.0.1:9432/postgres" > .env

./www/static:
	mkdir -p www/static

./data:
	mkdir data

dirs: www/static data .env

db: data
	podman run -d --replace --name=posadev_pg                    \
		-e POSTGRES_PASSWORD=prueba123                           \
		-v ./data:/var/lib/postgresql/data:U,Z                   \
		-p 127.0.0.1:9432:5432 ghcr.io/enterprisedb/postgresql:16

run: dirs db
	cargo run

clean:
	cargo clean
