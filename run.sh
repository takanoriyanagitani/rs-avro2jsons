#!/bin/sh

genavro(){
	export ENV_SCHEMA_FILENAME=sample.d/sample.avsc
	cat sample.d/records.jsonl |
		json2avrows |
		cat > ./sample.d/sample.avro
}

#genavro

cat sample.d/sample.avro |
	./rs-avro2jsons |
	jq -c
