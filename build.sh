if [ -d "utpm" ]; then
    cd utpm && 
    git pull origin main && 
    cargo install --path .
else
    git clone https://github.com/ThumusLive/utpm.git
    cd utpm && cargo install --path .
fi
