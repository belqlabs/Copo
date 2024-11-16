let it = 0;

setInterval(() => {
  console.log(`Meus argumentos:`);
  console.log(process.argv);
  console.log(`[${++it}]Estou rodando`);
}, 1000);
