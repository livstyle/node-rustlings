const {program, Command} = require('commander');

const rust = require('../');

const bootstrap = () => {
  program
    .version(
      require('../package.json').version,
      '-v, --version',
      'Output the current version.',
    )
    .usage('<command> [options]')
    .helpOption('-h, --help', 'Output usage information.');

    if (!process.argv.slice(2).length) {
      program.outputHelp();
    }

  program.command('list')
    .action(() => {
      rust.list();
    });

  program.command('watch')
    .action(() => {
      rust.watch();
    });  

  program.command('verify')
    .action(() => {
      rust.verify();
    }); 

  program.command('reset')
    .argument('<name>')
    .action((name) => {
      rust.reset(name);
    }); 

  program.command('run')
    .argument('<name>')
    .action((name) => {
      rust.run(name);
    }); 

  program.command('hint')
    .argument('<name>')
    .action((name) => {
      rust.hint(name);
    }); 

  program.command('lsp')
    .action(() => {
      rust.lsp();
    }); 

  program.parse(process.argv);
};
  
bootstrap();