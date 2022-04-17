#!/bin/sh

nvm use

rm aws/lambdas/attach_blurhash/bootstrap.zip
rm aws/lambdas/download_wallpaper_from_queue/bootstrap.zip
rm aws/lambdas/get_wallpapers_from_source/bootstrap.zip
rm aws/lambdas/search_wallpapers/bootstrap.zip
rm aws/stack.js


echo " - - - "
echo "  Compiling rust with --release flag now . . ." 
echo " - - - "
cross build --release --target x86_64-unknown-linux-musl

echo
echo " - - - "
echo "  Creating bootstrap of attach_blurhash . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/attach_blurhash ./bootstrap 
echo " - - - "
echo "  Creating zip of attach_blurhash . . ."
echo " - - - "
zip ./aws/lambdas/attach_blurhash/bootstrap.zip bootstrap     

rm bootstrap

echo
echo " - - - "
echo "  Creating bootstrap of download_wallpaper_from_queue . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/download_wallpaper_from_queue ./bootstrap 
echo " - - - "
echo "  Creating zip of download_wallpaper_from_queue . . ."
echo " - - - "
zip ./aws/lambdas/download_wallpaper_from_queue/bootstrap.zip bootstrap     

rm bootstrap

echo
echo " - - - "
echo "  Creating bootstrap of get_wallpapers_from_source . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/get_wallpapers_from_source ./bootstrap 
echo " - - - "
echo "  Creating zip of get_wallpapers_from_source . . ."
echo " - - - "
zip ./aws/lambdas/get_wallpapers_from_source/bootstrap.zip bootstrap     

rm bootstrap

echo
echo " - - - "
echo "  Creating bootstrap of search_wallpapers . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/search_wallpapers ./bootstrap 
echo " - - - "
echo "  Creating zip of search_wallpapers . . ."
echo " - - - "
zip ./aws/lambdas/search_wallpapers/bootstrap.zip bootstrap     

rm bootstrap

echo
echo " - - - "
echo "  Compiling ts for prod now . . ."
echo " - - - "
yarn build:aws:prod;