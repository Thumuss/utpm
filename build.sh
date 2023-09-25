if [ -d "utpm" ]; then
    cd utpm 
    git pull origin main && 
    cargo install --path . --bin utpm
else
    cargo install --git https://github.com/ThumusLive/utpm.git --bin utpm
fi
