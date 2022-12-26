//! A module for storing const-like variables like programs in different representations.

pub mod internal;

pub mod native {
    pub mod new {
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

        /// A string for testing commands.
        pub static COMMANDS: &str = "$^W\n^A~^D_^F ^S";

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
