### postgres csv dump


#### login
docker-compose exec postgresdb psql -d diesel_demo -U postgres

#### dump command
COPY (テーブル名) TO '(ファイルパス)' WITH CSV DELIMITER ',';

#### dump run
COPY photos TO '/var/tmp/photos.csv' WITH CSV HEADER


