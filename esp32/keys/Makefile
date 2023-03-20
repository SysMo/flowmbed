key/gen:
	keytool -genkey -keyalg RSA -alias hivemq -validity 360 -keysize 2048

pem/gen:
	openssl req -x509 -newkey rsa:2048 -keyout mqtt-client-key.pem -out mqtt-client-cert.pem -days 360