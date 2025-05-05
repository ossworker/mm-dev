git config --local gpg.x509.program gitsign # Use gitsign for signing  
git config --local gpg.format x509 # gitsign expects x509 args  
git config --local commit.gpgsign true # Sign all commits  
git config --local tag.gpgsign true # Sign all tags  
  
git config --local gitsign.connectorID https://github.com/login/oauth  
