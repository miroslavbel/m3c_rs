//! A module for storing const-like variables like programs in different representations.

pub mod internal;

pub mod native {
    pub mod new {
        /// Totally empty string.
        pub static EMPTY: &str = "";

        /// Contains only magic.
        pub static ONLY_MAGIC: &str = "$";

        /// Contains only one instruction (`MoveW`) and no magic.
        pub static NO_MAGIC_BUT_MOVE_W: &str = "^W";

        /// Contains some instructions and some illegal chars to start token with.
        pub static WITH_ILLEGAL_START_CHARS: &str = "$^W]]]]^Sфівіаві^F";

        /// Contains some instructions and some illegal chars in the middle of token.
        pub static WITH_UNKNOWN_CONTINUATION_CHARS: &str = "$^W^a^SGEa^FGE";

        /// A string with all `Simple` kind instuctions.
        pub static ALL_SIMPLE: &str = concat!(
            "$<|<-|<=|",
            "^F^W^D^S^A",
            "adswzghrbq,",
            "[F][W][WA][D][DW][S][SD][A][AS][r][l][f][w][d][s][a]",
            "=G=n=e=f=c=a=b=s=k=d=A",
            "=B=K=g=y=r=o=q=x=R",
            "=hp50=hp-",
            "#S#E",
            "B1;B3;B2;BEEP;RAND;VB;GEO;ZZ;POLY;C190;CRAFT;UP;NANO;REM;",
            "BUILD;DIGG;HEAL;MINE;",
            "AUT+AUT-AGR+AGR-",
            "ANDOR",
            "CCW;CW;",
            "FLIP;",
            "FILL;",
            "iaidisiw",
            "Hand+Hand-",
            "<|"
        );

        /// A string for testing commands. Not all commands are present.
        pub static NOT_ALL_COMMANDS: &str = "$^W\n^A~^D_^F ^S";

        /// A string for testing commands.
        pub static COMMANDS: &str = concat!(
            "$^W ^W  ^W_^W_ ^W\n_  ^W__^W\n__ ^W\n__  ^W\n___^W\n___ ^W\n___  ^W\n____^W\n____ ^W\n____  ^W\n_____^W~\n",
            "^D\n ^D\n\n  ^D\n.\n_^D\n..\n_ ^D~\n_  ^D\n...\n__^D\n.5.\n__ ^D~\n\n.0.\n__  ^D~\n~\n",
            "^S~\n~\n~\n^S~\n~\n~\n~\n~\n~\n^S"
        );

        /// A string for testing all not-`Simple` kind instruction.
        pub static LITERALS: &str = concat!(
            "$",
            "|:|hi:|012:",
            ">abc|:>zxc>->s12>=>sbf>",
            "!?if<?ifn<",
            "#Rrsp<",
            "(va0<0)(a=99999)(va2>-5)",
            "{dst}!{bp}"
        );
    }
}
