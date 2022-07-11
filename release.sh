mkdir -p release/src
#cat pom-prod.xml | grep -v remove-before-sending > release/pom.xml
cp -r src release/
cp Cargo.lock release/Cargo.lock
cp Cargo.toml release/Cargo.toml
cp entrypoint.sh release/entrypoint.sh
cp Dockerfile release/Dockerfile
cd release
zip -r version-$(date +"%m_%d_%Y-%H:%M").zip .