#!/bin/bash 
 
git clone https://github.com/kevlarxyz/kevlar-rs.wiki.git
cd kevlar-rs.wiki 
cp ../docs/* . 
git add . 
git commit -m "updating docs" 
git push 
cd ..
rm -rf kevlar-rs.wiki
