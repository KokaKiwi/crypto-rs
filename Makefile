RUST_MODULES		:=	crypto
RUSTCFLAGS			+=	--opt-level=3

include				rust-mk/rust.mk
