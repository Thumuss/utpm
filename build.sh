if [ -d "utpm" ]; then
    cd utpm 
    git pull origin main && 
    cargo install --path . --bin utpm
else
    cargo install --git https://github.com/Thumuss/utpm.git --bin utpm
fi
