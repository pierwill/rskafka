//! Error codes.
//!
//! # References
//! - <https://kafka.apache.org/protocol#protocol_error_codes>

use super::primitives::Int16;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    UnknownServerError,
    OffsetOutOfRange,
    CorruptMessage,
    UnknownTopicOrPartition,
    InvalidFetchSize,
    LeaderNotAvailable,
    NotLeaderOrFollower,
    RequestTimedOut,
    BrokerNotAvailable,
    ReplicaNotAvailable,
    MessageTooLarge,
    StaleControllerEpoch,
    OffsetMetadataTooLarge,
    NetworkException,
    CoordinatorLoadInProgress,
    CoordinatorNotAvailable,
    NotCoordinator,
    InvalidTopicException,
    RecordListTooLarge,
    NotEnoughReplicas,
    NotEnoughReplicasAfterAppend,
    InvalidRequiredAcks,
    IllegalGeneration,
    InconsistentGroupProtocol,
    InvalidGroupId,
    UnknownMemberId,
    InvalidSessionTimeout,
    RebalanceInProgress,
    InvalidCommitOffsetSize,
    TopicAuthorizationFailed,
    GroupAuthorizationFailed,
    ClusterAuthorizationFailed,
    InvalidTimestamp,
    UnsupportedSaslMechanism,
    IllegalSaslState,
    UnsupportedVersion,
    TopicAlreadyExists,
    InvalidPartitions,
    InvalidReplicationFactor,
    InvalidReplicaAssignment,
    InvalidConfig,
    NotController,
    InvalidRequest,
    UnsupportedForMessageFormat,
    PolicyViolation,
    OutOfOrderSequenceNumber,
    DuplicateSequenceNumber,
    InvalidProducerEpoch,
    InvalidTxnState,
    InvalidProducerIdMapping,
    InvalidTransactionTimeout,
    ConcurrentTransactions,
    TransactionCoordinatorFenced,
    TransactionalIdAuthorizationFailed,
    SecurityDisabled,
    OperationNotAttempted,
    KafkaStorageError,
    LogDirNotFound,
    SaslAuthenticationFailed,
    UnknownProducerId,
    ReassignmentInProgress,
    DelegationTokenAuthDisabled,
    DelegationTokenNotFound,
    DelegationTokenOwnerMismatch,
    DelegationTokenRequestNotAllowed,
    DelegationTokenAuthorizationFailed,
    DelegationTokenExpired,
    InvalidPrincipalType,
    NonEmptyGroup,
    GroupIdNotFound,
    FetchSessionIdNotFound,
    InvalidFetchSessionEpoch,
    ListenerNotFound,
    TopicDeletionDisabled,
    FencedLeaderEpoch,
    UnknownLeaderEpoch,
    UnsupportedCompressionType,
    StaleBrokerEpoch,
    OffsetNotAvailable,
    MemberIdRequired,
    PreferredLeaderNotAvailable,
    GroupMaxSizeReached,
    FencedInstanceId,
    EligibleLeadersNotAvailable,
    ElectionNotNeeded,
    NoReassignmentInProgress,
    GroupSubscribedToTopic,
    InvalidRecord,
    UnstableOffsetCommit,
    ThrottlingQuotaExceeded,
    ProducerFenced,
    ResourceNotFound,
    DuplicateResource,
    UnacceptableCredential,
    InconsistentVoterSet,
    InvalidUpdateVersion,
    FeatureUpdateFailed,
    PrincipalDeserializationFailure,
    SnapshotNotFound,
    PositionOutOfRange,
    UnknownTopicId,
    DuplicateBrokerRegistration,
    BrokerIdNotRegistered,
    InconsistentTopicId,
    InconsistentClusterId,
    TransactionalIdNotFound,
    Unknown(Int16),
}

impl Error {
    /// Coversion from [`Int16`].
    ///
    /// This cannot be a `From` trait due to <https://github.com/rust-lang/rfcs/issues/1402>.
    pub fn new(code: Int16) -> Option<Self> {
        match code.0 {
            -1 => Some(Self::UnknownServerError),
            0 => None,
            1 => Some(Self::OffsetOutOfRange),
            2 => Some(Self::CorruptMessage),
            3 => Some(Self::UnknownTopicOrPartition),
            4 => Some(Self::InvalidFetchSize),
            5 => Some(Self::LeaderNotAvailable),
            6 => Some(Self::NotLeaderOrFollower),
            7 => Some(Self::RequestTimedOut),
            8 => Some(Self::BrokerNotAvailable),
            9 => Some(Self::ReplicaNotAvailable),
            10 => Some(Self::MessageTooLarge),
            11 => Some(Self::StaleControllerEpoch),
            12 => Some(Self::OffsetMetadataTooLarge),
            13 => Some(Self::NetworkException),
            14 => Some(Self::CoordinatorLoadInProgress),
            15 => Some(Self::CoordinatorNotAvailable),
            16 => Some(Self::NotCoordinator),
            17 => Some(Self::InvalidTopicException),
            18 => Some(Self::RecordListTooLarge),
            19 => Some(Self::NotEnoughReplicas),
            20 => Some(Self::NotEnoughReplicasAfterAppend),
            21 => Some(Self::InvalidRequiredAcks),
            22 => Some(Self::IllegalGeneration),
            23 => Some(Self::InconsistentGroupProtocol),
            24 => Some(Self::InvalidGroupId),
            25 => Some(Self::UnknownMemberId),
            26 => Some(Self::InvalidSessionTimeout),
            27 => Some(Self::RebalanceInProgress),
            28 => Some(Self::InvalidCommitOffsetSize),
            29 => Some(Self::TopicAuthorizationFailed),
            30 => Some(Self::GroupAuthorizationFailed),
            31 => Some(Self::ClusterAuthorizationFailed),
            32 => Some(Self::InvalidTimestamp),
            33 => Some(Self::UnsupportedSaslMechanism),
            34 => Some(Self::IllegalSaslState),
            35 => Some(Self::UnsupportedVersion),
            36 => Some(Self::TopicAlreadyExists),
            37 => Some(Self::InvalidPartitions),
            38 => Some(Self::InvalidReplicationFactor),
            39 => Some(Self::InvalidReplicaAssignment),
            40 => Some(Self::InvalidConfig),
            41 => Some(Self::NotController),
            42 => Some(Self::InvalidRequest),
            43 => Some(Self::UnsupportedForMessageFormat),
            44 => Some(Self::PolicyViolation),
            45 => Some(Self::OutOfOrderSequenceNumber),
            46 => Some(Self::DuplicateSequenceNumber),
            47 => Some(Self::InvalidProducerEpoch),
            48 => Some(Self::InvalidTxnState),
            49 => Some(Self::InvalidProducerIdMapping),
            50 => Some(Self::InvalidTransactionTimeout),
            51 => Some(Self::ConcurrentTransactions),
            52 => Some(Self::TransactionCoordinatorFenced),
            53 => Some(Self::TransactionalIdAuthorizationFailed),
            54 => Some(Self::SecurityDisabled),
            55 => Some(Self::OperationNotAttempted),
            56 => Some(Self::KafkaStorageError),
            57 => Some(Self::LogDirNotFound),
            58 => Some(Self::SaslAuthenticationFailed),
            59 => Some(Self::UnknownProducerId),
            60 => Some(Self::ReassignmentInProgress),
            61 => Some(Self::DelegationTokenAuthDisabled),
            62 => Some(Self::DelegationTokenNotFound),
            63 => Some(Self::DelegationTokenOwnerMismatch),
            64 => Some(Self::DelegationTokenRequestNotAllowed),
            65 => Some(Self::DelegationTokenAuthorizationFailed),
            66 => Some(Self::DelegationTokenExpired),
            67 => Some(Self::InvalidPrincipalType),
            68 => Some(Self::NonEmptyGroup),
            69 => Some(Self::GroupIdNotFound),
            70 => Some(Self::FetchSessionIdNotFound),
            71 => Some(Self::InvalidFetchSessionEpoch),
            72 => Some(Self::ListenerNotFound),
            73 => Some(Self::TopicDeletionDisabled),
            74 => Some(Self::FencedLeaderEpoch),
            75 => Some(Self::UnknownLeaderEpoch),
            76 => Some(Self::UnsupportedCompressionType),
            77 => Some(Self::StaleBrokerEpoch),
            78 => Some(Self::OffsetNotAvailable),
            79 => Some(Self::MemberIdRequired),
            80 => Some(Self::PreferredLeaderNotAvailable),
            81 => Some(Self::GroupMaxSizeReached),
            82 => Some(Self::FencedInstanceId),
            83 => Some(Self::EligibleLeadersNotAvailable),
            84 => Some(Self::ElectionNotNeeded),
            85 => Some(Self::NoReassignmentInProgress),
            86 => Some(Self::GroupSubscribedToTopic),
            87 => Some(Self::InvalidRecord),
            88 => Some(Self::UnstableOffsetCommit),
            89 => Some(Self::ThrottlingQuotaExceeded),
            90 => Some(Self::ProducerFenced),
            91 => Some(Self::ResourceNotFound),
            92 => Some(Self::DuplicateResource),
            93 => Some(Self::UnacceptableCredential),
            94 => Some(Self::InconsistentVoterSet),
            95 => Some(Self::InvalidUpdateVersion),
            96 => Some(Self::FeatureUpdateFailed),
            97 => Some(Self::PrincipalDeserializationFailure),
            98 => Some(Self::SnapshotNotFound),
            99 => Some(Self::PositionOutOfRange),
            100 => Some(Self::UnknownTopicId),
            101 => Some(Self::DuplicateBrokerRegistration),
            102 => Some(Self::BrokerIdNotRegistered),
            103 => Some(Self::InconsistentTopicId),
            104 => Some(Self::InconsistentClusterId),
            105 => Some(Self::TransactionalIdNotFound),
            _ => Some(Self::Unknown(code)),
        }
    }
}

impl From<Option<Error>> for Int16 {
    fn from(error: Option<Error>) -> Self {
        let error = match error {
            Some(error) => error,
            None => return Int16(0),
        };

        match error {
            Error::UnknownServerError => Int16(-1),
            Error::OffsetOutOfRange => Int16(1),
            Error::CorruptMessage => Int16(2),
            Error::UnknownTopicOrPartition => Int16(3),
            Error::InvalidFetchSize => Int16(4),
            Error::LeaderNotAvailable => Int16(5),
            Error::NotLeaderOrFollower => Int16(6),
            Error::RequestTimedOut => Int16(7),
            Error::BrokerNotAvailable => Int16(8),
            Error::ReplicaNotAvailable => Int16(9),
            Error::MessageTooLarge => Int16(10),
            Error::StaleControllerEpoch => Int16(11),
            Error::OffsetMetadataTooLarge => Int16(12),
            Error::NetworkException => Int16(13),
            Error::CoordinatorLoadInProgress => Int16(14),
            Error::CoordinatorNotAvailable => Int16(15),
            Error::NotCoordinator => Int16(16),
            Error::InvalidTopicException => Int16(17),
            Error::RecordListTooLarge => Int16(18),
            Error::NotEnoughReplicas => Int16(19),
            Error::NotEnoughReplicasAfterAppend => Int16(20),
            Error::InvalidRequiredAcks => Int16(21),
            Error::IllegalGeneration => Int16(22),
            Error::InconsistentGroupProtocol => Int16(23),
            Error::InvalidGroupId => Int16(24),
            Error::UnknownMemberId => Int16(25),
            Error::InvalidSessionTimeout => Int16(26),
            Error::RebalanceInProgress => Int16(27),
            Error::InvalidCommitOffsetSize => Int16(28),
            Error::TopicAuthorizationFailed => Int16(29),
            Error::GroupAuthorizationFailed => Int16(30),
            Error::ClusterAuthorizationFailed => Int16(31),
            Error::InvalidTimestamp => Int16(32),
            Error::UnsupportedSaslMechanism => Int16(33),
            Error::IllegalSaslState => Int16(34),
            Error::UnsupportedVersion => Int16(35),
            Error::TopicAlreadyExists => Int16(36),
            Error::InvalidPartitions => Int16(37),
            Error::InvalidReplicationFactor => Int16(38),
            Error::InvalidReplicaAssignment => Int16(39),
            Error::InvalidConfig => Int16(40),
            Error::NotController => Int16(41),
            Error::InvalidRequest => Int16(42),
            Error::UnsupportedForMessageFormat => Int16(43),
            Error::PolicyViolation => Int16(44),
            Error::OutOfOrderSequenceNumber => Int16(45),
            Error::DuplicateSequenceNumber => Int16(46),
            Error::InvalidProducerEpoch => Int16(47),
            Error::InvalidTxnState => Int16(48),
            Error::InvalidProducerIdMapping => Int16(49),
            Error::InvalidTransactionTimeout => Int16(50),
            Error::ConcurrentTransactions => Int16(51),
            Error::TransactionCoordinatorFenced => Int16(52),
            Error::TransactionalIdAuthorizationFailed => Int16(53),
            Error::SecurityDisabled => Int16(54),
            Error::OperationNotAttempted => Int16(55),
            Error::KafkaStorageError => Int16(56),
            Error::LogDirNotFound => Int16(57),
            Error::SaslAuthenticationFailed => Int16(58),
            Error::UnknownProducerId => Int16(59),
            Error::ReassignmentInProgress => Int16(60),
            Error::DelegationTokenAuthDisabled => Int16(61),
            Error::DelegationTokenNotFound => Int16(62),
            Error::DelegationTokenOwnerMismatch => Int16(63),
            Error::DelegationTokenRequestNotAllowed => Int16(64),
            Error::DelegationTokenAuthorizationFailed => Int16(65),
            Error::DelegationTokenExpired => Int16(66),
            Error::InvalidPrincipalType => Int16(67),
            Error::NonEmptyGroup => Int16(68),
            Error::GroupIdNotFound => Int16(69),
            Error::FetchSessionIdNotFound => Int16(70),
            Error::InvalidFetchSessionEpoch => Int16(71),
            Error::ListenerNotFound => Int16(72),
            Error::TopicDeletionDisabled => Int16(73),
            Error::FencedLeaderEpoch => Int16(74),
            Error::UnknownLeaderEpoch => Int16(75),
            Error::UnsupportedCompressionType => Int16(76),
            Error::StaleBrokerEpoch => Int16(77),
            Error::OffsetNotAvailable => Int16(78),
            Error::MemberIdRequired => Int16(79),
            Error::PreferredLeaderNotAvailable => Int16(80),
            Error::GroupMaxSizeReached => Int16(81),
            Error::FencedInstanceId => Int16(82),
            Error::EligibleLeadersNotAvailable => Int16(83),
            Error::ElectionNotNeeded => Int16(84),
            Error::NoReassignmentInProgress => Int16(85),
            Error::GroupSubscribedToTopic => Int16(86),
            Error::InvalidRecord => Int16(87),
            Error::UnstableOffsetCommit => Int16(88),
            Error::ThrottlingQuotaExceeded => Int16(89),
            Error::ProducerFenced => Int16(90),
            Error::ResourceNotFound => Int16(91),
            Error::DuplicateResource => Int16(92),
            Error::UnacceptableCredential => Int16(93),
            Error::InconsistentVoterSet => Int16(94),
            Error::InvalidUpdateVersion => Int16(95),
            Error::FeatureUpdateFailed => Int16(96),
            Error::PrincipalDeserializationFailure => Int16(97),
            Error::SnapshotNotFound => Int16(98),
            Error::PositionOutOfRange => Int16(99),
            Error::UnknownTopicId => Int16(100),
            Error::DuplicateBrokerRegistration => Int16(101),
            Error::BrokerIdNotRegistered => Int16(102),
            Error::InconsistentTopicId => Int16(103),
            Error::InconsistentClusterId => Int16(104),
            Error::TransactionalIdNotFound => Int16(105),
            Error::Unknown(code) => code,
        }
    }
}
