const BYTE_LEN: usize = 8;

#[cfg(target_arch = "x86_64")]
pub const ARCH_WORD_LEN: usize = 64;

#[cfg(target_arch = "x86")]
pub const ARCH_WORD_LEN: usize = 32;

#[cfg(target_arch = "x86_64")]
pub const ARCH_WORD_BYTES: usize = ARCH_WORD_LEN / BYTE_LEN;

#[cfg(target_arch = "x86")]
pub const ARCH_WORD_BYTES: usize = X32 / 8;

#[cfg(target_arch = "x86_64")]
pub type RegisterType = u64;

#[cfg(target_arch = "x86")]
pub type RegisterType = u32;

pub const RISC_V_32_INSTRUCTION_BYTES: usize = 4;
pub const RISC_V_32_REGISTERS: usize = 32;
pub type R32I = u32;


struct Bytes<const N: usize>([Byte; N]);

impl<const N: usize> Bytes<N> {
    const fn get<const I: usize>(&self) -> Byte {
        assert!(I < N);
        self.0[I]
    }
}

type Word64 = Bytes<ARCH_WORD_BYTES>;
type DWord128 = Bytes<{ 2 * ARCH_WORD_BYTES }>;

impl<const N: usize> Bytes<N> {
    const fn zero() -> Bytes<N> {
        Bytes([0; N])
    }
}

impl<const N: usize> Default for Bytes<N> {
    fn default() -> Self {
        Bytes([Byte::default(); N])
    }
}

pub type Byte = u8;
pub type Address = u32;

pub const PC_DEFAULT_ADDRESS: Address = 0;
pub const STACK_DEFAULT_ADDRESS: Address = 1 << 10;
pub const PC_STEP: Address = 4;