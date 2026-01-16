FROM ghcr.io/pyo3/maturin:latest

COPY . /home
WORKDIR /home

RUN python3 -m venv .env && source /home/.env/bin/activate

ENTRYPOINT "sh"
CMD "sh source /home/.env/bin/activate && sh"