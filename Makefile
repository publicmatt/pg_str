PACKAGE_VERSION=0.3.0
PACKAGE_NAME=pg_str
PG_VERSION=15
PREFIX=target/release/$(PACKAGE_NAME)-pg$(PG_VERSION)

build:
	cargo pgx package

install: $(PREFIX)
	cp -f $(PREFIX)/usr/share/postgresql/$(PG_VERSION)/extension/$(PACKAGE_NAME)--$(PACKAGE_VERSION).sql /usr/share/postgresql/$(PG_VERSION)/extension/$(PACKAGE_NAME)--$(PACKAGE_VERSION).sql
	cp -f $(PREFIX)/usr/lib/postgresql/$(PG_VERSION)/lib/$(PACKAGE_NAME).so /usr/lib/postgresql/$(PG_VERSION)/lib/$(PACKAGE_NAME).so
	cp -f $(PREFIX)/usr/share/postgresql/$(PG_VERSION)/extension/$(PACKAGE_NAME).control /usr/share/postgresql/$(PG_VERSION)/extension/$(PACKAGE_NAME).control

restart:
	sudo service postgresql restart
