#!/bin/sh

nvm use

rm aws/lambdas/attach_blurhash/bootstrap.zip
rm aws/lambdas/authorizer/bootstrap.zip
rm aws/lambdas/download_image_from_queue/bootstrap.zip
rm aws/lambdas/get_amoled_backgrounds_from_source/bootstrap.zip
rm aws/lambdas/get_image/bootstrap.zip
rm aws/lambdas/get_wallpapers_from_source/bootstrap.zip
rm aws/lambdas/ignore_image_toggle/bootstrap.zip
rm aws/lambdas/login/bootstrap.zip
rm aws/lambdas/logout/bootstrap.zip
rm aws/lambdas/search_images/bootstrap.zip
rm aws/stack.js


echo " - - - "
echo "  Compiling rust with --release flag now . . ." 
echo " - - - "
cross build --release --target x86_64-unknown-linux-musl

echo
echo " - - - "
echo "  Creating bootstrap of authorizer . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/authorizer ./bootstrap 
echo " - - - "
echo "  Creating zip of authorizer . . ."
echo " - - - "
zip ./aws/lambdas/authorizer/bootstrap.zip bootstrap     

rm bootstrap

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
echo "  Creating bootstrap of download_image_from_queue . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/download_image_from_queue ./bootstrap 
echo " - - - "
echo "  Creating zip of download_image_from_queue . . ."
echo " - - - "
zip ./aws/lambdas/download_image_from_queue/bootstrap.zip bootstrap     

rm bootstrap

echo
echo " - - - "
echo "  Creating bootstrap of get_image . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/get_image ./bootstrap 
echo " - - - "
echo "  Creating zip of get_image . . ."
echo " - - - "
zip ./aws/lambdas/get_image/bootstrap.zip bootstrap     

rm bootstrap

echo
echo " - - - "
echo "  Creating bootstrap of get_amoled_backgrounds_from_source . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/get_amoled_backgrounds_from_source ./bootstrap 
echo " - - - "
echo "  Creating zip of get_amoled_backgrounds_from_source . . ."
echo " - - - "
zip ./aws/lambdas/get_amoled_backgrounds_from_source/bootstrap.zip bootstrap     

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
echo "  Creating bootstrap of ignore_image_toggle . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/ignore_image_toggle ./bootstrap 
echo " - - - "
echo "  Creating zip of ignore_image_toggle . . ."
echo " - - - "
zip ./aws/lambdas/ignore_image_toggle/bootstrap.zip bootstrap     

rm bootstrap

echo
echo " - - - "
echo "  Creating bootstrap of login . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/login ./bootstrap 
echo " - - - "
echo "  Creating zip of login . . ."
echo " - - - "
zip ./aws/lambdas/login/bootstrap.zip bootstrap     

rm bootstrap

echo
echo " - - - "
echo "  Creating bootstrap of logout . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/logout ./bootstrap 
echo " - - - "
echo "  Creating zip of logout . . ."
echo " - - - "
zip ./aws/lambdas/logout/bootstrap.zip bootstrap     

rm bootstrap

echo
echo " - - - "
echo "  Creating bootstrap of search_images . . ."
echo " - - - "
cp target/x86_64-unknown-linux-musl/release/search_images ./bootstrap 
echo " - - - "
echo "  Creating zip of search_images . . ."
echo " - - - "
zip ./aws/lambdas/search_images/bootstrap.zip bootstrap     

rm bootstrap

echo
echo " - - - "
echo "  Compiling ts for prod now . . ."
echo " - - - "
yarn build:aws:prod;