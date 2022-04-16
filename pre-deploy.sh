#!/bin/sh

nvm use

rm aws/lambdas/attach_blurhash/bootstrap.zip
rm aws/lambdas/download_wallpaper_from_queue/bootstrap.zip
rm aws/lambdas/get_wallpapers_from_source/bootstrap.zip
rm aws/stack.js


echo "Compiling rust now . . ." 
cross build --release --target x86_64-unknown-linux-musl

echo "Creating bootstrap of attach_blurhash . . ."
cp target/x86_64-unknown-linux-musl/release/attach_blurhash ./bootstrap 
echo "Creating zip of attach_blurhash . . ."
zip ./aws/lambdas/attach_blurhash/bootstrap.zip bootstrap     

rm bootstrap

echo "Creating bootstrap of download_wallpaper_from_queue . . ."
cp target/x86_64-unknown-linux-musl/release/download_wallpaper_from_queue ./bootstrap 
echo "Creating zip of download_wallpaper_from_queue . . ."
zip ./aws/lambdas/download_wallpaper_from_queue/bootstrap.zip bootstrap     

rm bootstrap

echo "Creating bootstrap of get_wallpapers_from_source . . ."
cp target/x86_64-unknown-linux-musl/release/get_wallpapers_from_source ./bootstrap 
echo "Creating zip of get_wallpapers_from_source . . ."
zip ./aws/lambdas/get_wallpapers_from_source/bootstrap.zip bootstrap     

rm bootstrap

yarn build:aws:prod;