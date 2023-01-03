use crate::element::Element;
use crate::vhdl::keywords::*;
use crate::vhdl::design_unit::DesignUnit;
use crate::vhdl::block_declarative_item::BlockDeclarativeItem;
use crate::vhdl::constant_declaration::ConstantDeclaration;
use crate::vhdl::signal_declaration::SignalDeclaraion;
use crate::vhdl::concurrent_statement::ConcurrentStatement;
use crate::vhdl::signal_assignment::SignalAssignment;

pub struct Architecture {
    name : String,
    entity : String,
    declarations : Vec< Box< dyn BlockDeclarativeItem > >,
    statements : Vec< Box< dyn ConcurrentStatement > >
}

impl Architecture {
    pub fn new( name : & str, entity : & str ) -> Architecture {
        Architecture { name : name.to_string(), entity : entity.to_string(),
                declarations : Vec::new(), statements : Vec::new() }
    }

    pub fn add_constant_declaration( & mut self, constant_declaration : ConstantDeclaration ) {
        self.declarations.push( Box::< ConstantDeclaration >::new( constant_declaration ) );
    }

    pub fn add_signal_declaration( & mut self, signal_declaration : SignalDeclaraion ) {
        self.declarations.push( Box::< SignalDeclaraion >::new( signal_declaration ) );
    }

    pub fn add_signal_assignment( & mut self, signal_assignment : SignalAssignment ) {
        self.statements.push( Box::< SignalAssignment >::new( signal_assignment ) );
    }
}

impl Element for Architecture {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );

        source.push_str( & format!( "{}{} {} {} {} {}\n", indent_str, ARCHITECTURE, self.name, OF,
                self.entity, IS ) );
        for declaration in & self.declarations {
            source.push_str( & declaration.to_source_code( indent + 1 ) );
        }
        source.push_str( & format!( "{}{}\n", indent_str, BEGIN ) );
        for statement in & self.statements {
            source.push_str( & statement.to_source_code( indent + 1 ) );
        }
        source.push_str( & format!( "{}{} {} {};\n", indent_str, END, ARCHITECTURE, self.name ) );

        return source;
    }
}

impl DesignUnit for Architecture {
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    const NAME : &'static str = "rtl";
    const ENTITY : &'static str = "test";
    const HEADER : &'static str = "architecture rtl of test is\n";
    const BEGIN : &'static str = "begin\n";
    const END : &'static str = "end architecture rtl;\n";
    const SIGNAL_DECLARATION : &'static str = "    signal signal_1 : boolean;\n";
    const SIGNAL_ASSIGNMENT : &'static str = "    s1: signal_1 <= true;\n";
    const CONSTANT_DECLARATION : &'static str = "    constant const_1 : integer := 12;\n";

    /**
     * Create a architecture with no content.
     */
    #[test]
    fn architecture_frame() {
        let architecture = Architecture::new( NAME, ENTITY );

        assert_eq!(
            architecture.to_source_code( 0 ),
            format!( "{}{}{}", HEADER, BEGIN, END )
        );
    }

    /**
     * Create a architecture with a singal declaration.
     */
    #[test]
    fn architecture_with_signal_declaration() {
        let mut architecture = Architecture::new( NAME, ENTITY );
        architecture.add_signal_declaration( SignalDeclaraion::new( "signal_1", "boolean" ) );

        assert_eq!(
            architecture.to_source_code( 0 ),
            format!( "{}{}{}{}", HEADER, SIGNAL_DECLARATION, BEGIN, END )
        );
    }

    /**
     * Create a architecture with a singal declaration and assignment.
     */
    #[test]
    fn architecture_with_signal_declaration_and_assignment() {
        let mut architecture = Architecture::new( NAME, ENTITY );
        architecture.add_signal_declaration( SignalDeclaraion::new( "signal_1", "boolean" ) );
        architecture.add_signal_assignment( SignalAssignment::new_with_label( "s1", "signal_1", "true" ) );

        assert_eq!(
            architecture.to_source_code( 0 ),
            format!( "{}{}{}{}{}", HEADER, SIGNAL_DECLARATION, BEGIN, SIGNAL_ASSIGNMENT, END )
        );
    }

    /**
     * Create a architecture with a constant declaration.
     */
    #[test]
    fn architecture_with_constant_declaration_and_assignment() {
        let mut architecture = Architecture::new( NAME, ENTITY );
        architecture.add_constant_declaration( ConstantDeclaration::new( "const_1", "integer", "12" ) );

        assert_eq!(
            architecture.to_source_code( 0 ),
            format!( "{}{}{}{}", HEADER, CONSTANT_DECLARATION, BEGIN, END )
        );
    }
}

