app_data_dir=(dirname $1)

mkdir -pv $app_data_dir

find ./src-tauri/migrations -type f -name "*.sql" -exec sqlite3 "${1}" ".read {}" \;
