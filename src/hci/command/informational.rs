use hci::opcode::InformationalOpcode;


define_command!(InformationalOpcode::ReadLocalVersionInformation);
define_command!(InformationalOpcode::ReadLocalSupportedCommands);
define_command!(InformationalOpcode::ReadLocalSupportedFeatures);
define_command!(InformationalOpcode::ReadLocalExtendedFeatures, page: u8);
define_command!(InformationalOpcode::ReadBufferSize);
define_command!(InformationalOpcode::ReadBdAddr);
define_command!(InformationalOpcode::ReadDataBlockSize);
