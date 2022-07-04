 FROM node:carbon

 WORKDIR '/myapp'

 RUN git clone https://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:x-oauth-basic@github.com/stanis18/front-verification-tool.git /myapp/

 RUN npm install

 EXPOSE 3000


 CMD ["npm", "start"]

# To run
# docker container run -i -t -p 3000:3000 nome_da_imagem

# To build
# docker build -t nome_da_imagem .