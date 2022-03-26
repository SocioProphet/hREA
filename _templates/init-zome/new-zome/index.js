// see types of prompts:
// https://github.com/enquirer/enquirer/tree/master/examples
//
module.exports = [
  {
    type: 'input',
    name: 'dna_path',
    message: 'Project-relative directory name of the destination DNA? (eg. `bundles/dna/observation`)',
    required: true,
  }, {
    type: 'input',
    name: 'zome_name',
    message: 'Name of the new zome? (eg. `rea_economic_event`)',
    required: true,
  }, {
    type: 'input',
    name: 'zome_friendly_name',
    message: 'Human-readable short name for the zome, to use in file comments (eg. "Holo-REA economic event")',
    required: true,
  }, {
    type: 'input',
    name: 'zome_description',
    message: 'Human-readable description for this zome, to show to users in the hApp store',
    required: true,
  }, {
    type: 'input',
    name: 'package_author_name',
    message: 'Initial author name for published Rust crate?',
    required: true,
  }, {
    type: 'input',
    name: 'package_author_email',
    message: 'Initial author email address for published Rust crate?',
    required: true,
  }, {
    type: 'input',
    name: 'record_type_name',
    message: 'Type name to use for the primary record exposed by this zome\'s API? (eg. `Economic Event`)',
    required: true,
  },
]
