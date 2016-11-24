// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// Bindings for a subset of sdk/simplelink/include/socket.h

#[repr(i16)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum Family {
    AF_INET = 2,
    AF_INET6 = 3,
    AF_RF = 6,
    AF_INET6_EUI_48 = 9,
    AF_PACKET = 17,
}

#[repr(i16)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum Protocol {
    DEFAULT = 0,
    IPPROTO_TCP = 6, // TCP Raw Socket
    IPPROTO_UDP = 17, // UDP Raw Socket
    IPPROTO_RAW = 255, // Raw Socket
}

#[repr(i16)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum SocketType {
    SOCK_STREAM = 1, // TCP Socket
    SOCK_DGRAM = 2, // UDP Socket
    SOCK_RAW = 3, // RAW Socket
}

#[repr(i16)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SocketError {
    SOC_ERROR = -1, // Failure.
    SOC_OK = 0, // Success.
    INEXE = -8, // socket command in execution
    EBADF = -9, // Bad file number
    ENSOCK = -10, // The system limit on the total number of open socket, has been reached
    EAGAIN = -11, // Try again
    // EWOULDBLOCK = -11,
    ENOMEM = -12, // Out of memory
    EACCES = -13, // Permission denied
    EFAULT = -14, // Bad address
    ECLOSE = -15, // close socket operation failed to transmit all queued packets
    EALREADY_ENABLED = -21, // Transceiver - Transceiver already ON. there could be only one
    EINVAL = -22, // Invalid argument
    EAUTO_CONNECT_OR_CONNECTING = -69, /* Transceiver - During connection, connected or auto mode started */
    CONNECTION_PENDING = -72, /* Transceiver - Device is connected, disconnect first to open transceiver */
    EUNSUPPORTED_ROLE = -86, // Transceiver - Trying to start when WLAN role is AP or P2P GO
    EDESTADDRREQ = -89, // Destination address required
    EPROTOTYPE = -91, // Protocol wrong type for socket
    ENOPROTOOPT = -92, // Protocol not available
    EPROTONOSUPPORT = -93, // Protocol not supported
    ESOCKTNOSUPPORT = -94, // Socket type not supported
    EOPNOTSUPP = -95, // Operation not supported on transport endpoint
    EAFNOSUPPORT = -97, // Address family not supported by protocol
    EADDRINUSE = -98, // Address already in use
    EADDRNOTAVAIL = -99, // Cannot assign requested address
    ENETUNREACH = -101, // Network is unreachable
    ENOBUFS = -105, // No buffer space available
    // EOBUFF = -105,
    EISCONN = -106, // Transport endpoint is already connected
    ENOTCONN = -107, // Transport endpoint is not connected
    ETIMEDOUT = -110, // Connection timed out
    ECONNREFUSED = -111, // Connection refused
    EALREADY = -114, // Non blocking connect in progress, try again

    ESEC_RSA_WRONG_TYPE_E = -130, // RSA wrong block type for RSA function
    ESEC_RSA_BUFFER_E = -131, // RSA buffer error, output too small or
    ESEC_BUFFER_E = -132, // output buffer too small or input too large
    ESEC_ALGO_ID_E = -133, // setting algo id error
    ESEC_PUBLIC_KEY_E = -134, // setting public key error
    ESEC_DATE_E = -135, // setting date validity error
    ESEC_SUBJECT_E = -136, // setting subject name error
    ESEC_ISSUER_E = -137, // setting issuer  name error
    ESEC_CA_TRUE_E = -138, // setting CA basic constraint true error
    ESEC_EXTENSIONS_E = -139, // setting extensions error
    ESEC_ASN_PARSE_E = -140, // ASN parsing error, invalid input
    ESEC_ASN_VERSION_E = -141, // ASN version error, invalid number
    ESEC_ASN_GETINT_E = -142, // ASN get big _i16 error, invalid data
    ESEC_ASN_RSA_KEY_E = -143, // ASN key init error, invalid input
    ESEC_ASN_OBJECT_ID_E = -144, // ASN object id error, invalid id
    ESEC_ASN_TAG_NULL_E = -145, // ASN tag error, not null
    ESEC_ASN_EXPECT_0_E = -146, // ASN expect error, not zero
    ESEC_ASN_BITSTR_E = -147, // ASN bit string error, wrong id
    ESEC_ASN_UNKNOWN_OID_E = -148, // ASN oid error, unknown sum id
    ESEC_ASN_DATE_SZ_E = -149, // ASN date error, bad size
    ESEC_ASN_BEFORE_DATE_E = -150, // ASN date error, current date before
    ESEC_ASN_AFTER_DATE_E = -151, // ASN date error, current date after
    ESEC_ASN_SIG_OID_E = -152, // ASN signature error, mismatched oid
    ESEC_ASN_TIME_E = -153, // ASN time error, unknown time type
    ESEC_ASN_INPUT_E = -154, // ASN input error, not enough data
    ESEC_ASN_SIG_CONFIRM_E = -155, // ASN sig error, confirm failure
    ESEC_ASN_SIG_HASH_E = -156, // ASN sig error, unsupported hash type
    ESEC_ASN_SIG_KEY_E = -157, // ASN sig error, unsupported key type
    ESEC_ASN_DH_KEY_E = -158, // ASN key init error, invalid input
    ESEC_ASN_NTRU_KEY_E = -159, // ASN ntru key decode error, invalid input
    ESEC_ECC_BAD_ARG_E = -170, // ECC input argument of wrong type
    ESEC_ASN_ECC_KEY_E = -171, // ASN ECC bad input
    ESEC_ECC_CURVE_OID_E = -172, // Unsupported ECC OID curve type
    ESEC_BAD_FUNC_ARG = -173, // Bad function argument provided
    ESEC_NOT_COMPILED_IN = -174, // Feature not compiled in
    ESEC_UNICODE_SIZE_E = -175, // Unicode password too big
    ESEC_NO_PASSWORD = -176, // no password provided by user
    ESEC_ALT_NAME_E = -177, // alt name size problem, too big
    ESEC_AES_GCM_AUTH_E = -180, // AES-GCM Authentication check failure
    ESEC_AES_CCM_AUTH_E = -181, // AES-CCM Authentication check failure
    SOCKET_ERROR_E = -208, // Error state on socket

    ESEC_MEMORY_ERROR = -203, // out of memory
    ESEC_VERIFY_FINISHED_ERROR = -204, // verify problem on finished
    ESEC_VERIFY_MAC_ERROR = -205, // verify mac problem
    ESEC_UNKNOWN_HANDSHAKE_TYPE = -207, // weird handshake type
    // ESEC_SOCKET_ERROR_E = -208, // error state on socket
    ESEC_SOCKET_NODATA = -209, // expected data, not there
    ESEC_INCOMPLETE_DATA = -210, // don't have enough data to complete task
    ESEC_UNKNOWN_RECORD_TYPE = -211, // unknown type in record hdr
    ESEC_FATAL_ERROR = -213, // recvd alert fatal error
    ESEC_ENCRYPT_ERROR = -214, // error during encryption
    ESEC_NO_PEER_KEY = -216, // need peer's key
    ESEC_NO_PRIVATE_KEY = -217, // need the private key
    ESEC_RSA_PRIVATE_ERROR = -218, // error during rsa priv op
    ESEC_NO_DH_PARAMS = -219, // server missing DH params
    ESEC_BUILD_MSG_ERROR = -220, // build message failure
    ESEC_BAD_HELLO = -221, // client hello malformed
    ESEC_DOMAIN_NAME_MISMATCH = -222, // peer subject name mismatch
    ESEC_WANT_READ = -223, // want read, call again
    ESEC_NOT_READY_ERROR = -224, // handshake layer not ready
    ESEC_PMS_VERSION_ERROR = -225, // pre m secret version error
    ESEC_VERSION_ERROR = -226, // record layer version error
    ESEC_WANT_WRITE = -227, // want write, call again
    ESEC_BUFFER_ERROR = -228, // malformed buffer input
    ESEC_VERIFY_CERT_ERROR = -229, // verify cert error
    ESEC_VERIFY_SIGN_ERROR = -230, // verify sign error

    ESEC_LENGTH_ERROR = -241, // record layer length error
    ESEC_PEER_KEY_ERROR = -242, // can't decode peer key
    ESEC_ZERO_RETURN = -243, // peer sent close notify
    ESEC_SIDE_ERROR = -244, // wrong client/server type
    ESEC_NO_PEER_CERT = -245, // peer didn't send key
    ESEC_ECC_CURVETYPE_ERROR = -250, // Bad ECC Curve Type
    ESEC_ECC_CURVE_ERROR = -251, // Bad ECC Curve
    ESEC_ECC_PEERKEY_ERROR = -252, // Bad Peer ECC Key
    ESEC_ECC_MAKEKEY_ERROR = -253, // Bad Make ECC Key
    ESEC_ECC_EXPORT_ERROR = -254, // Bad ECC Export Key
    ESEC_ECC_SHARED_ERROR = -255, // Bad ECC Shared Secret
    ESEC_NOT_CA_ERROR = -257, // Not a CA cert error
    ESEC_BAD_PATH_ERROR = -258, // Bad path for opendir
    ESEC_BAD_CERT_MANAGER_ERROR = -259, // Bad Cert Manager
    ESEC_MAX_CHAIN_ERROR = -268, // max chain depth exceeded
    ESEC_SUITES_ERROR = -271, // suites pointer error
    ESEC_SSL_NO_PEM_HEADER = -272, // no PEM header found
    ESEC_OUT_OF_ORDER_E = -273, // out of order message
    ESEC_SANITY_CIPHER_E = -275, // sanity check on cipher error
    ESEC_GEN_COOKIE_E = -277, // Generate Cookie Error
    ESEC_NO_PEER_VERIFY = -278, // Need peer cert verify Error
    ESEC_UNKNOWN_SNI_HOST_NAME_E = -281, // Unrecognized host name Error
    // begin negotiation parameter errors
    ESEC_UNSUPPORTED_SUITE = -290, // unsupported cipher suite
    ESEC_MATCH_SUITE_ERROR = -291, // can't match cipher suite

    // ssl tls security start with -300 offset
    ESEC_CLOSE_NOTIFY = -300, // ssl/tls alerts
    ESEC_UNEXPECTED_MESSAGE = -310, // ssl/tls alerts
    ESEC_BAD_RECORD_MAC = -320, // ssl/tls alerts
    ESEC_DECRYPTION_FAILED = -321, // ssl/tls alerts
    ESEC_RECORD_OVERFLOW = -322, // ssl/tls alerts
    ESEC_DECOMPRESSION_FAILURE = -330, // ssl/tls alerts
    ESEC_HANDSHAKE_FAILURE = -340, // ssl/tls alerts
    ESEC_NO_CERTIFICATE = -341, // ssl/tls alerts
    ESEC_BAD_CERTIFICATE = -342, // ssl/tls alerts
    ESEC_UNSUPPORTED_CERTIFICATE = -343, // ssl/tls alerts
    ESEC_CERTIFICATE_REVOKED = -344, // ssl/tls alerts
    ESEC_CERTIFICATE_EXPIRED = -345, // ssl/tls alerts
    ESEC_CERTIFICATE_UNKNOWN = -346, // ssl/tls alerts
    ESEC_ILLEGAL_PARAMETER = -347, // ssl/tls alerts
    ESEC_UNKNOWN_CA = -348, // ssl/tls alerts
    ESEC_ACCESS_DENIED = -349, // ssl/tls alerts
    ESEC_DECODE_ERROR = -350, // ssl/tls alerts
    ESEC_DECRYPT_ERROR = -351, // ssl/tls alerts
    ESEC_EXPORT_RESTRICTION = -360, // ssl/tls alerts
    ESEC_PROTOCOL_VERSION = -370, // ssl/tls alerts
    ESEC_INSUFFICIENT_SECURITY = -371, // ssl/tls alerts
    ESEC_INTERNAL_ERROR = -380, // ssl/tls alerts
    ESEC_USER_CANCELLED = -390, // ssl/tls alerts
    ESEC_NO_RENEGOTIATION = -400, // ssl/tls alerts
    ESEC_UNSUPPORTED_EXTENSION = -410, // ssl/tls alerts
    ESEC_CERTIFICATE_UNOBTAINABLE = -411, // ssl/tls alerts
    ESEC_UNRECOGNIZED_NAME = -412, // ssl/tls alerts
    ESEC_BAD_CERTIFICATE_STATUS_RESPONSE = -413, // ssl/tls alerts
    ESEC_BAD_CERTIFICATE_HASH_VALUE = -414, // ssl/tls alerts
    // propierty secure
    ESECGENERAL = -450, // error secure level general error
    ESECDECRYPT = -451, // error secure level, decrypt recv packet fail
    ESECCLOSED = -452, // secure layrer is closed by other size , tcp is still connected
    ESECSNOVERIFY = -453, // Connected without server verification
    ESECNOCAFILE = -454, // error secure level CA file not found
    ESECMEMORY = -455, // error secure level No memory  space available
    ESECBADCAFILE = -456, // error secure level bad CA file
    ESECBADCERTFILE = -457, // error secure level bad Certificate file
    ESECBADPRIVATEFILE = -458, // error secure level bad private file
    ESECBADDHFILE = -459, // error secure level bad DH file
    ESECT00MANYSSLOPENED = -460, // MAX SSL Sockets are opened
    ESECDATEERROR = -461, // connected with certificate date verification error
    ESECHANDSHAKETIMEDOUT = -462, // connection timed out due to handshake time
}

pub type RawSocket = i16;

// Casts a RawSocket into an Error to check the return value of sl_Socket.
impl ::core::convert::Into<SocketError> for RawSocket {
    fn into(self) -> SocketError {
        unsafe { ::core::intrinsics::transmute::<RawSocket, SocketError>(self) }
    }
}

// Used for return types that are either an error or a buffer size.
pub type SizeOrError = i16;

// Used to convert a SizeOf Error into a Result for functions like sl_Send
impl ::core::convert::TryInto<SocketError> for SizeOrError {
    type Err = ();
    fn try_into(self) -> Result<SocketError, Self::Err> {
        if self >= 0 {
            return  Err(());
        }
        Ok(unsafe { ::core::intrinsics::transmute::<SizeOrError, SocketError>(self) })
    }
}

// Enum for the return type of SL_FD_ISSET
#[repr(i16)]
#[allow(non_camel_case_types)]
pub enum SlBoolean {
    True = 1,
    False = 0,
}

#[allow(non_camel_case_types)]
pub type SlSocklen_t = i16;

// A Socket address data structure.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SlSockAddr_t {
    pub family: Family,
    pub data: [u8; 14usize],
}

impl ::core::default::Default for SlSockAddr_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct SlSockAddrIn_t {
    pub sin_family: Family,
    pub sin_port: u16,
    pub sin_addr: u32, // ip address in network byte order.
    pub sin_zero: [u8; 8usize],
}

impl ::core::default::Default for SlSockAddrIn_t {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct SlFdSet_t {
    pub fd_array: [u32; 1usize],
}

impl ::core::default::Default for SlFdSet_t {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

pub type SltimeT = u32;
pub type SlsusecondsT = u32;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct SlTimeval_t {
    pub tv_sec: SltimeT,
    pub tv_usec: SlsusecondsT,
}

impl ::core::default::Default for SlTimeval_t {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

#[repr(i16)]
#[allow(non_camel_case_types)]
pub enum OptionLevel {
    SOL_SOCKET = 1, // Define the socket option category.
    IPPROTO_IP = 2, // Define the IP option category.
    SOL_PHY_OPT = 3, // Define the PHY option category.
}

#[repr(i16)]
#[allow(non_camel_case_types)]
pub enum OptionName {
    SO_RCVBUF = 8, // Setting TCP receive buffer size
    SO_KEEPALIVE = 9, // Connections are kept alive with periodic messages
    SO_RCVTIMEO = 20, // Enable receive timeout
    SO_NONBLOCKING = 24, // Enable . disable nonblocking mode
    SO_SECMETHOD = 25, // security metohd
    SO_SECURE_MASK = 26, // security mask
    SO_SECURE_FILES = 27, // security files
    SO_CHANGE_CHANNEL = 28, // This option is available only when transceiver started
    SO_SECURE_FILES_PRIVATE_KEY_FILE_NAME = 30, // This option used to configue secure file
    SO_SECURE_FILES_CERTIFICATE_FILE_NAME = 31, // This option used to configue secure file
    SO_SECURE_FILES_CA_FILE_NAME = 32, // This option used to configue secure file
    SO_SECURE_FILES_DH_KEY_FILE_NAME = 33, // This option used to configue secure file
    SO_SECURE_DOMAIN_NAME_VERIFICATION = 35,

    IP_MULTICAST_IF = 60, // Specify outgoing multicast interface
    IP_MULTICAST_TTL = 61, // Specify the TTL value to use for outgoing multicast packet.
    IP_ADD_MEMBERSHIP = 65, // Join IPv4 multicast membership
    IP_DROP_MEMBERSHIP = 66, // Leave IPv4 multicast membership
    IP_HDRINCL = 67, // Raw socket IPv4 header included.
    IP_RAW_RX_NO_HEADER = 68, /* Proprietary socket option that does not includeIPv4/IPv6 header=and extension headers) on received raw sockets */
    IP_RAW_IPV6_HDRINCL = 69, // Transmitted buffer over IPv6 socket contains IPv6 header.

    SO_PHY_RATE = 100, // WLAN Transmit rate
    SO_PHY_TX_POWER = 101, // TX Power level
    SO_PHY_NUM_FRAMES_TO_TX = 102, // Number of frames to transmit
    SO_PHY_PREAMBLE = 103, // Preamble for transmission
}

extern "C" {
    pub fn sl_Htonl(val: u32) -> u32;
    pub fn sl_Htons(val: u16) -> u16;

    /// create an endpoint for communication
    ///
    /// The socket function creates a new socket of a certain socket type, identified
    /// by an integer number, and allocates system resources to it.
    /// This function is called by the application layer to obtain a socket handle.
    ///
    /// domain           specifies the protocol family of the created socket.
    ///                        For example:
    ///                           AF_INET for network protocol IPv4
    ///                           AF_RF for starting transceiver mode. Notes:
    ///                           - sending and receiving any packet overriding 802.11 header
    ///                           - for optimized power consumption the socket will be started in TX
    ///                             only mode until receive command is activated
    ///                           AF_INET6 for IPv6
    ///
    /// socket_type              specifies the communication semantic, one of:
    ///                           SOCK_STREAM=reliable stream-oriented service or Stream Sockets)
    ///                           SOCK_DGRAM=datagram service or Datagram Sockets)
    ///                           SOCK_RAW=raw protocols atop the network layer)
    ///                           when used with AF_RF:
    ///                                   SOCK_DGRAM - L2 socket
    ///                                   SOCK_RAW - L1 socket - bypass WLAN CCA=Clear Channel Assessment)
    ///
    /// protocol         specifies a particular transport to be used with the socket.
    ///                        The most common are IPPROTO_TCP, IPPROTO_SCTP, IPPROTO_UDP,
    ///                        IPPROTO_DCCP.
    ///                        The value 0 may be used to select a default
    ///                        protocol from the selected domain and type
    ///
    ///  On success, socket handle that is used for consequent socket operations.
    ///                        A successful return code should be a positive number=int16)
    ///                        On error, a negative=int16) value will be returned specifying the error code.
    ///                   AFNOSUPPORT  - illegal domain parameter
    ///                   PROTOTYPE  - illegal type parameter
    ///                   ACCES   - permission denied
    ///                   NSOCK  - exceeded maximal number of socket
    ///                   NOMEM  - memory allocation error
    ///                   INVAL  - error in socket configuration
    ///                   PROTONOSUPPORT  - illegal protocol parameter
    ///                   OPNOTSUPP  - illegal combination of protocol and type parameters
    pub fn sl_Socket(family: Family, socket_type: SocketType, protocol: Protocol) -> RawSocket;

    /// This function causes the system to release resources allocated to a socket.
    /// In case of TCP, the connection is terminated.
    ///
    /// socket   socket handle=received in sl_Socket)
    ///
    /// On success, zero is returned.
    /// On error, a negative number is returned.
    pub fn sl_Close(socket: RawSocket) -> SocketError;

    /// Initiate a connection on a socket
    ///
    /// Function connects the socket referred to by the socket
    /// descriptor sd, to the address specified by addr. The addrlen
    /// argument specifies the size of addr. The format of the
    /// address in addr is determined by the address space of the
    /// socket. If it is of type SOCK_DGRAM, this call specifies the
    /// peer with which the socket is to be associated; this address
    /// is that to which datagrams are to be sent, and the only
    /// address from which datagrams are to be received.  If the
    /// socket is of type SOCK_STREAM, this call attempts to make a
    /// connection to another socket. The other socket is specified
    /// by address, which is an address in the communications space
    /// of the socket.
    ///
    /// socket           socket descriptor=handle)
    /// addr             specifies the destination addr sockaddr:
    ///                        - code for the address format. On this version only AF_INET is supported.
    ///                        - socket address, the length depends on the code format
    ///
    /// addrlen          contains the size of the structure pointed to by addr
    ///
    ///                  On success, returns a socket handle.
    ///                     On a non-blocking connect a possible negative value is SL_EALREADY.
    ///                     On failure, negative value.
    ///                        SL_POOL_IS_EMPTY may be return in case there are no resources in the system
    ///                          In this case try again later or increase MAX_CONCURRENT_ACTIONS
    pub fn sl_Connect(socket: RawSocket, addr: *const SlSockAddr_t, addrlen: i16) -> SocketError;

    /// set socket options
    ///
    /// This function manipulate the options associated with a socket.
    /// Options may exist at multiple protocol levels; they are always
    /// present at the uppermost socket level.
    ///
    /// When manipulating socket options the level at which the option resides
    /// and the name of the option must be specified.  To manipulate options at
    /// the socket level, level is specified as SOL_SOCKET.  To manipulate
    /// options at any other level the protocol number of the appropriate proto-
    /// col controlling the option is supplied.  For example, to indicate that an
    /// option is to be interpreted by the TCP protocol, level should be set to
    /// the protocol number of TCP;
    ///
    /// The parameters optval and optlen are used to access optval
    /// values for setsockopt().  For getsockopt() they identify a
    /// buffer in which the value for the requested option(s) are to
    /// be returned.  For getsockopt(), optlen is a value-result
    /// parameter, initially containing the size of the buffer
    /// pointed to by option_value, and modified on return to
    /// indicate the actual size of the value returned.  If no option
    /// value is to be supplied or returned, option_value may be
    /// NULL.
    ///
    /// socket               socket handle
    /// level            defines the protocol level for this option
    ///                        - SOL_SOCKET   Socket level configurations=L4, transport layer)
    ///                        - IPPROTO_IP   IP level configurations=L3, network layer)
    ///                        - SOL_PHY_OPT  Link level configurations=L2, link layer)
    /// optname          defines the option name to interrogate
    ///                        - SL_SOL_SOCKET
    ///                          - SL_SO_KEEPALIVE
    ///                                         Enable/Disable periodic keep alive.
    ///                                         Keeps TCP connections active by enabling the periodic transmission of messages
    ///                                         Timeout is 5 minutes.
    ///                                         Default: Enabled
    ///                                         This options takes SlSockKeepalive_t struct as parameter
    ///                          - SL_SO_RCVTIMEO
    ///                                         Sets the timeout value that specifies the maximum amount of time an input function waits until it completes.
    ///                                         Default: No timeout
    ///                                         This options takes SlTimeval_t struct as parameter
    ///                          - SL_SO_RCVBUF
    ///                                         Sets tcp max recv window size.
    ///                                         This options takes SlSockWinsize_t struct as parameter
    ///                          - SL_SO_NONBLOCKING
    ///                                         Sets socket to non-blocking operation Impacts: connect, accept, send, sendto, recv and recvfrom.
    ///                                         Default: Blocking.
    ///                                         This options takes SlSockNonblocking_t struct as parameter
    ///                          - SL_SO_SECMETHOD
    ///                                         Sets method to tcp secured socket=SL_SEC_SOCKET)
    ///                                         Default: SL_SO_SEC_METHOD_SSLv3_TLSV1_2
    ///                                         This options takes SlSockSecureMethod struct as parameter
    ///                          - SL_SO_SEC_MASK
    ///                                     Sets specific cipher to tcp secured socket=SL_SEC_SOCKET)
    ///                                         Default: "Best" cipher suitable to method
    ///                                         This options takes SlSockSecureMask struct as parameter
    ///                          - SL_SO_SECURE_FILES_CA_FILE_NAME
    ///                                         Map secured socket to CA file by name
    ///                                         This options takes _u8 buffer as parameter
    ///                          - SL_SO_SECURE_FILES_PRIVATE_KEY_FILE_NAME
    ///                                         Map secured socket to private key by name
    ///                                         This options takes _u8 buffer as parameter
    ///                          - SL_SO_SECURE_FILES_CERTIFICATE_FILE_NAME
    ///                                         Map secured socket to certificate file by name
    ///                                         This options takes _u8 buffer as parameter
    ///                          - SL_SO_SECURE_FILES_DH_KEY_FILE_NAME
    ///                                         Map secured socket to Diffie Hellman file by name
    ///                                         This options takes _u8 buffer as parameter
    ///                          - SL_SO_CHANGE_CHANNEL
    ///                                         Sets channel in transceiver mode.
    ///                                         This options takes _u32 as channel number parameter
    ///                        - SL_IPPROTO_IP
    ///                          - SL_IP_MULTICAST_TTL
    ///                                         Set the time-to-live value of outgoing multicast packets for this socket.
    ///                                         This options takes _u8 as parameter
    ///                          - SL_IP_ADD_MEMBERSHIP
    ///                                         UDP socket, Join a multicast group.
    ///                                         This options takes SlSockIpMreq struct as parameter
    ///                          - SL_IP_DROP_MEMBERSHIP
    ///                                         UDP socket, Leave a multicast group
    ///                                         This options takes SlSockIpMreq struct as parameter
    ///                          - SL_IP_RAW_RX_NO_HEADER
    ///                                         Raw socket remove IP header from received data.
    ///                                         Default: data includes ip header
    ///                                         This options takes _u32 as parameter
    ///                          - SL_IP_HDRINCL
    ///                                         RAW socket only, the IPv4 layer generates an IP header when sending a packet unless
    ///                                         the IP_HDRINCL socket option is enabled on the socket.
    ///                                         When it is enabled, the packet must contain an IP header.
    ///                                         Default: disabled, IPv4 header generated by Network Stack
    ///                                         This options takes _u32 as parameter
    ///                          - SL_IP_RAW_IPV6_HDRINCL=inactive)
    ///                                         RAW socket only, the IPv6 layer generates an IP header when sending a packet unless
    ///                                         the IP_HDRINCL socket option is enabled on the socket. When it is enabled, the packet must contain an IP header
    ///                                         Default: disabled, IPv4 header generated by Network Stack
    ///                                         This options takes _u32 as parameter
    ///                        - SL_SOL_PHY_OPT
    ///                          - SL_SO_PHY_RATE
    ///                                         RAW socket, set WLAN PHY transmit rate
    ///                                         The values are based on RateIndex_e
    ///                                         This options takes _u32 as parameter
    ///                          - SL_SO_PHY_TX_POWER
    ///                                         RAW socket, set WLAN PHY TX power
    ///                                         Valid rage is 1-15
    ///                                         This options takes _u32 as parameter
    ///                          - SL_SO_PHY_NUM_FRAMES_TO_TX
    ///                                         RAW socket, set number of frames to transmit in transceiver mode.
    ///                                         Default: 1 packet
    ///                                         This options takes _u32 as parameter
    ///                          - SL_SO_PHY_PREAMBLE
    ///                                         RAW socket, set WLAN PHY preamble for Long/Short
    ///                                         This options takes _u32 as parameter
    ///
    /// optval           specifies a value for the option
    /// optlen           specifies the length of the option value
    ///
    ///                     On success, zero is returned.
    ///                     On error, a negative value is returned.
    pub fn sl_SetSockOpt(socket: RawSocket,
                         level: OptionLevel,
                         optname: OptionName,
                         optval: *const u8,
                         optlen: SlSocklen_t)
                         -> SocketError;

    /// Get socket options
    ///
    /// This function manipulate the options associated with a socket.
    /// Options may exist at multiple protocol levels; they are always
    /// present at the uppermost socket level.
    ///
    /// When manipulating socket options the level at which the option resides
    /// and the name of the option must be specified.  To manipulate options at
    /// the socket level, level is specified as SOL_SOCKET.  To manipulate
    /// options at any other level the protocol number of the appropriate proto-
    /// col controlling the option is supplied.  For example, to indicate that an
    /// option is to be interpreted by the TCP protocol, level should be set to
    /// the protocol number of TCP;
    ///
    /// The parameters optval and optlen are used to access optval -
    /// ues for setsockopt().  For getsockopt() they identify a
    /// buffer in which the value for the requested option(s) are to
    /// be returned.  For getsockopt(), optlen is a value-result
    /// parameter, initially containing the size of the buffer
    /// pointed to by option_value, and modified on return to
    /// indicate the actual size of the value returned.  If no option
    /// value is to be supplied or returned, option_value may be
    /// NULL.
    ///
    /// socket          socket handle
    /// level           defines the protocol level for this option
    /// optname         defines the option name to interrogate
    /// optval          specifies a value for the option
    /// optlen          specifies the length of the option value
    ///
    ///             On success, zero is returned.
    ///             On error, a negative value is returned.
    pub fn sl_GetSockOpt(socket: RawSocket,
                         level: OptionLevel,
                         optname: OptionName,
                         optval: *mut u8,
                         optlen: *mut SlSocklen_t)
                         -> SocketError;

    /// read data from TCP socket
    ///
    /// function receives a message from a connection-mode socket
    ///
    /// socket              socket handle
    ///  buf             Points to the buffer where the message should be stored.
    ///  Len             Specifies the length in bytes of
    ///                  the buffer pointed to by the buffer argument.
    ///                  Range: 1-16000 bytes
    ///  flags           Specifies the type of message
    ///                  reception. On this version, this parameter is not supported.
    ///
    ///                  return the number of bytes received,
    ///                  or a negative value if an error occurred.
    ///                  using a non-blocking recv a possible negative value is SL_EAGAIN.
    ///                  SL_POOL_IS_EMPTY may be return in case there are no resources in the system
    ///                 In this case try again later or increase MAX_CONCURRENT_ACTIONS
    pub fn sl_Recv(socket: RawSocket, buf: *mut u8, len: i16, flags: i16) -> SizeOrError;

    /// write data to TCP socket
    ///
    /// This function is used to transmit a message to another socket.
    /// Returns immediately after sending data to device.
    /// In case of TCP failure an async event SL_SOCKET_TX_FAILED_EVENT is going to be received.
    /// In case of a RAW socket (transceiver mode), extra 4 bytes should be reserved at the end of the
    /// frame data buffer for WLAN FCS
    ///
    /// socket           socket handle
    /// buf              Points to a buffer containing the message to be sent
    /// len              message size in bytes. Range: 1-1460 bytes
    /// flags            Specifies the type of message
    ///                        transmission. On this version, this parameter is not
    ///                        supported for TCP.
    ///                        For transceiver mode, the SL_RAW_RF_TX_PARAMS macro can be used to determine
    ///                        transmission parameters (channel,rate,tx_power,preamble)
    ///
    ///                   Return the number of bytes transmitted, or -1 if an error occurred
    pub fn sl_Send(socket: RawSocket,
                   buf: *const u8,
                   len: i16,
                   flags: i16) -> SizeOrError;

    /// assign a name to a socket
    ///
    /// This function gives the socket the local address addr.
    /// addr is addrlen bytes long. Traditionally, this is called
    /// When a socket is created with socket, it exists in a name
    /// space (address family) but has no name assigned.
    ///  It is necessary to assign a local address before a SOCK_STREAM
    ///  socket may receive connections.
    ///
    ///  socket            socket descriptor (handle)
    ///  addr              specifies the destination addrs sockaddr:
    ///                       - code for
    ///                         the address format. On this
    ///                         version only AF_INET is
    ///                         supported.
    ///                       - socket address,
    ///                         the length depends on the code
    ///                         format
    /// addrlen          contains the size of the structure pointed to by addr
    ///
    ///                 On success, zero is returned. On error, a negative error code is returned.
    pub fn sl_Bind(socket: RawSocket, addr: *const SlSockAddr_t, addrlen: i16) -> SocketError;

    ///  listen for connections on a socket
    ///
    /// The willingness to accept incoming connections and a queue
    /// limit for incoming connections are specified with listen(),
    /// and then the connections are accepted with accept.
    /// The listen() call applies only to sockets of type SOCK_STREAM
    /// The backlog parameter defines the maximum length the queue of
    /// pending connections may grow to.
    ///
    /// socket            socket descriptor (handle)
    /// backlog          specifies the listen queue depth.
    ///
    ///          On success, zero is returned. On error, a negative error code is returned.
    pub fn sl_Listen(socket: RawSocket, backlog: i16) -> SocketError;

    ///  Monitor socket activity
    ///
    ///  Select allow a program to monitor multiple file descriptors,
    ///  waiting until one or more of the file descriptors become
    ///  "ready" for some class of I/O operation
    ///
    /// nfds        the highest-numbered file descriptor in any of the three sets, plus 1.
    /// readsds     socket descriptors list for read monitoring and accept monitoring
    /// writesds    socket descriptors list for connect monitoring only, write monitoring is not supported, non blocking connect is supported
    /// exceptsds   socket descriptors list for exception monitoring, not supported.
    /// timeout     is an upper bound on the amount of time elapsed
    ///             before select() returns. Null or above 0xffff seconds means
    ///             infinity timeout. The minimum timeout is 10 milliseconds,
    ///             less than 10 milliseconds will be set automatically to 10 milliseconds.
    ///             Max microseconds supported is 0xfffc00.
    ///
    /// return          On success, select()  returns the number of
    ///                 file descriptors contained in the three returned
    ///                 descriptor sets (that is, the total number of bits that
    ///                 are set in readfds, writefds, exceptfds) which may be
    ///                 zero if the timeout expires before anything interesting
    ///                 happens. On error, a negative value is returned.
    ///                 readsds - return the sockets on which Read request will
    ///                 return without delay with valid data.
    ///                 writesds - return the sockets on which Write request
    ///                 will return without delay.
    ///                 exceptsds - return the sockets closed recently.
    ///                 SL_POOL_IS_EMPTY may be return in case there are no resources in the system
    ///                 In this case try again later or increase MAX_CONCURRENT_ACTIONS
    pub fn sl_Select(nfds: i16, readsds: *mut SlFdSet_t,
                     writesds: *mut SlFdSet_t, exceptsds: *mut SlFdSet_t,
                     timeout: *mut SlTimeval_t) -> SizeOrError;

    /// Sets current socket descriptor on SlFdSet_t container
    pub fn SL_FD_SET(socket: RawSocket, fdset: *mut SlFdSet_t);

    /// Clears current socket descriptor on SlFdSet_t container
    pub fn SL_FD_CLR(socket: RawSocket, fdset: *mut SlFdSet_t);

    /// Checks if current socket descriptor is set (TRUE/FALSE)
    pub fn SL_FD_ISSET(socket: RawSocket, fdset: *mut SlFdSet_t) -> SlBoolean;

    // Clears all socket descriptors from SlFdSet_t
    pub fn SL_FD_ZERO(fdset: *mut SlFdSet_t);
}
