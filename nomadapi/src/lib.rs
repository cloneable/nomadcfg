#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AclPolicy {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "Description",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<String>,
        #[serde(rename = "JobACL", default, skip_serializing_if = "Option::is_none")]
        pub job_acl: Option<JobAcl>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Rules", default, skip_serializing_if = "Option::is_none")]
        pub rules: Option<String>,
    }

    impl From<&AclPolicy> for AclPolicy {
        fn from(value: &AclPolicy) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AclPolicyListStub {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "Description",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&AclPolicyListStub> for AclPolicyListStub {
        fn from(value: &AclPolicyListStub) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AclToken {
        #[serde(
            rename = "AccessorID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub accessor_id: Option<String>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "CreateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(
            rename = "ExpirationTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub expiration_time: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(
            rename = "ExpirationTTL",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub expiration_ttl: Option<i64>,
        #[serde(rename = "Global", default, skip_serializing_if = "Option::is_none")]
        pub global: Option<bool>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Policies", default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<String>,
        #[serde(rename = "Roles", default, skip_serializing_if = "Vec::is_empty")]
        pub roles: Vec<AclTokenRoleLink>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&AclToken> for AclToken {
        fn from(value: &AclToken) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AclTokenListStub {
        #[serde(
            rename = "AccessorID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub accessor_id: Option<String>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "CreateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration_time: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(rename = "Global", default, skip_serializing_if = "Option::is_none")]
        pub global: Option<bool>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Policies", default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<String>,
        #[serde(rename = "Roles", default, skip_serializing_if = "Vec::is_empty")]
        pub roles: Vec<AclTokenRoleLink>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&AclTokenListStub> for AclTokenListStub {
        fn from(value: &AclTokenListStub) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AclTokenRoleLink {
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&AclTokenRoleLink> for AclTokenRoleLink {
        fn from(value: &AclTokenRoleLink) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Affinity {
        #[serde(rename = "LTarget", default, skip_serializing_if = "Option::is_none")]
        pub l_target: Option<String>,
        #[serde(rename = "Operand", default, skip_serializing_if = "Option::is_none")]
        pub operand: Option<String>,
        #[serde(rename = "RTarget", default, skip_serializing_if = "Option::is_none")]
        pub r_target: Option<String>,
        #[serde(rename = "Weight", default, skip_serializing_if = "Option::is_none")]
        pub weight: Option<i8>,
    }

    impl From<&Affinity> for Affinity {
        fn from(value: &Affinity) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocDeploymentStatus {
        #[serde(rename = "Canary", default, skip_serializing_if = "Option::is_none")]
        pub canary: Option<bool>,
        #[serde(rename = "Healthy", default, skip_serializing_if = "Option::is_none")]
        pub healthy: Option<bool>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Timestamp", default, skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    }

    impl From<&AllocDeploymentStatus> for AllocDeploymentStatus {
        fn from(value: &AllocDeploymentStatus) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocStopResponse {
        #[serde(rename = "EvalID", default, skip_serializing_if = "Option::is_none")]
        pub eval_id: Option<String>,
        #[serde(rename = "Index", default, skip_serializing_if = "Option::is_none")]
        pub index: Option<u64>,
    }

    impl From<&AllocStopResponse> for AllocStopResponse {
        fn from(value: &AllocStopResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocatedCpuResources {
        #[serde(rename = "CpuShares", default, skip_serializing_if = "Option::is_none")]
        pub cpu_shares: Option<i64>,
    }

    impl From<&AllocatedCpuResources> for AllocatedCpuResources {
        fn from(value: &AllocatedCpuResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocatedDeviceResource {
        #[serde(rename = "DeviceIDs", default, skip_serializing_if = "Vec::is_empty")]
        pub device_i_ds: Vec<String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(rename = "Vendor", default, skip_serializing_if = "Option::is_none")]
        pub vendor: Option<String>,
    }

    impl From<&AllocatedDeviceResource> for AllocatedDeviceResource {
        fn from(value: &AllocatedDeviceResource) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocatedMemoryResources {
        #[serde(
            rename = "MemoryMaxMB",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub memory_max_mb: Option<i64>,
        #[serde(rename = "MemoryMB", default, skip_serializing_if = "Option::is_none")]
        pub memory_mb: Option<i64>,
    }

    impl From<&AllocatedMemoryResources> for AllocatedMemoryResources {
        fn from(value: &AllocatedMemoryResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocatedResources {
        #[serde(rename = "Shared", default, skip_serializing_if = "Option::is_none")]
        pub shared: Option<AllocatedSharedResources>,
        #[serde(
            rename = "Tasks",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub tasks: std::collections::HashMap<String, AllocatedTaskResources>,
    }

    impl From<&AllocatedResources> for AllocatedResources {
        fn from(value: &AllocatedResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocatedSharedResources {
        #[serde(rename = "DiskMB", default, skip_serializing_if = "Option::is_none")]
        pub disk_mb: Option<i64>,
        #[serde(rename = "Networks", default, skip_serializing_if = "Vec::is_empty")]
        pub networks: Vec<NetworkResource>,
        #[serde(rename = "Ports", default, skip_serializing_if = "Vec::is_empty")]
        pub ports: Vec<PortMapping>,
    }

    impl From<&AllocatedSharedResources> for AllocatedSharedResources {
        fn from(value: &AllocatedSharedResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocatedTaskResources {
        #[serde(rename = "Cpu", default, skip_serializing_if = "Option::is_none")]
        pub cpu: Option<AllocatedCpuResources>,
        #[serde(rename = "Devices", default, skip_serializing_if = "Vec::is_empty")]
        pub devices: Vec<AllocatedDeviceResource>,
        #[serde(rename = "Memory", default, skip_serializing_if = "Option::is_none")]
        pub memory: Option<AllocatedMemoryResources>,
        #[serde(rename = "Networks", default, skip_serializing_if = "Vec::is_empty")]
        pub networks: Vec<NetworkResource>,
    }

    impl From<&AllocatedTaskResources> for AllocatedTaskResources {
        fn from(value: &AllocatedTaskResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Allocation {
        #[serde(
            rename = "AllocModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub alloc_modify_index: Option<u64>,
        #[serde(
            rename = "AllocatedResources",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allocated_resources: Option<AllocatedResources>,
        #[serde(
            rename = "ClientDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_description: Option<String>,
        #[serde(
            rename = "ClientStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_status: Option<String>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "CreateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_time: Option<i64>,
        #[serde(
            rename = "DeploymentID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deployment_id: Option<String>,
        #[serde(
            rename = "DeploymentStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deployment_status: Option<AllocDeploymentStatus>,
        #[serde(
            rename = "DesiredDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub desired_description: Option<String>,
        #[serde(
            rename = "DesiredStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub desired_status: Option<String>,
        #[serde(
            rename = "DesiredTransition",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub desired_transition: Option<DesiredTransition>,
        #[serde(rename = "EvalID", default, skip_serializing_if = "Option::is_none")]
        pub eval_id: Option<String>,
        #[serde(
            rename = "FollowupEvalID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub followup_eval_id: Option<String>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "Job", default, skip_serializing_if = "Option::is_none")]
        pub job: Option<Job>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(rename = "Metrics", default, skip_serializing_if = "Option::is_none")]
        pub metrics: Option<AllocationMetric>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "ModifyTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_time: Option<i64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "NextAllocation",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub next_allocation: Option<String>,
        #[serde(rename = "NodeID", default, skip_serializing_if = "Option::is_none")]
        pub node_id: Option<String>,
        #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
        pub node_name: Option<String>,
        #[serde(
            rename = "PreemptedAllocations",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub preempted_allocations: Vec<String>,
        #[serde(
            rename = "PreemptedByAllocation",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub preempted_by_allocation: Option<String>,
        #[serde(
            rename = "PreviousAllocation",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub previous_allocation: Option<String>,
        #[serde(
            rename = "RescheduleTracker",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reschedule_tracker: Option<RescheduleTracker>,
        #[serde(rename = "Resources", default, skip_serializing_if = "Option::is_none")]
        pub resources: Option<Resources>,
        #[serde(
            rename = "Services",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub services: std::collections::HashMap<String, String>,
        #[serde(rename = "TaskGroup", default, skip_serializing_if = "Option::is_none")]
        pub task_group: Option<String>,
        #[serde(
            rename = "TaskResources",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub task_resources: std::collections::HashMap<String, Resources>,
        #[serde(
            rename = "TaskStates",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub task_states: std::collections::HashMap<String, TaskState>,
    }

    impl From<&Allocation> for Allocation {
        fn from(value: &Allocation) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocationListStub {
        #[serde(
            rename = "AllocatedResources",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allocated_resources: Option<AllocatedResources>,
        #[serde(
            rename = "ClientDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_description: Option<String>,
        #[serde(
            rename = "ClientStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_status: Option<String>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "CreateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_time: Option<i64>,
        #[serde(
            rename = "DeploymentStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deployment_status: Option<AllocDeploymentStatus>,
        #[serde(
            rename = "DesiredDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub desired_description: Option<String>,
        #[serde(
            rename = "DesiredStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub desired_status: Option<String>,
        #[serde(rename = "EvalID", default, skip_serializing_if = "Option::is_none")]
        pub eval_id: Option<String>,
        #[serde(
            rename = "FollowupEvalID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub followup_eval_id: Option<String>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(rename = "JobType", default, skip_serializing_if = "Option::is_none")]
        pub job_type: Option<String>,
        #[serde(
            rename = "JobVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_version: Option<u64>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "ModifyTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_time: Option<i64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "NodeID", default, skip_serializing_if = "Option::is_none")]
        pub node_id: Option<String>,
        #[serde(rename = "NodeName", default, skip_serializing_if = "Option::is_none")]
        pub node_name: Option<String>,
        #[serde(
            rename = "PreemptedAllocations",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub preempted_allocations: Vec<String>,
        #[serde(
            rename = "PreemptedByAllocation",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub preempted_by_allocation: Option<String>,
        #[serde(
            rename = "RescheduleTracker",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reschedule_tracker: Option<RescheduleTracker>,
        #[serde(rename = "TaskGroup", default, skip_serializing_if = "Option::is_none")]
        pub task_group: Option<String>,
        #[serde(
            rename = "TaskStates",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub task_states: std::collections::HashMap<String, TaskState>,
    }

    impl From<&AllocationListStub> for AllocationListStub {
        fn from(value: &AllocationListStub) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocationMetric {
        #[serde(
            rename = "AllocationTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allocation_time: Option<i64>,
        #[serde(
            rename = "ClassExhausted",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub class_exhausted: std::collections::HashMap<String, i64>,
        #[serde(
            rename = "ClassFiltered",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub class_filtered: std::collections::HashMap<String, i64>,
        #[serde(
            rename = "CoalescedFailures",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub coalesced_failures: Option<i64>,
        #[serde(
            rename = "ConstraintFiltered",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub constraint_filtered: std::collections::HashMap<String, i64>,
        #[serde(
            rename = "DimensionExhausted",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub dimension_exhausted: std::collections::HashMap<String, i64>,
        #[serde(
            rename = "NodesAvailable",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub nodes_available: std::collections::HashMap<String, i64>,
        #[serde(
            rename = "NodesEvaluated",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_evaluated: Option<i64>,
        #[serde(
            rename = "NodesExhausted",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_exhausted: Option<i64>,
        #[serde(
            rename = "NodesFiltered",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_filtered: Option<i64>,
        #[serde(
            rename = "QuotaExhausted",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub quota_exhausted: Vec<String>,
        #[serde(
            rename = "ResourcesExhausted",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub resources_exhausted: std::collections::HashMap<String, Resources>,
        #[serde(
            rename = "ScoreMetaData",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub score_meta_data: Vec<NodeScoreMeta>,
        #[serde(
            rename = "Scores",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub scores: std::collections::HashMap<String, f64>,
    }

    impl From<&AllocationMetric> for AllocationMetric {
        fn from(value: &AllocationMetric) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Attribute {
        #[serde(rename = "Bool", default, skip_serializing_if = "Option::is_none")]
        pub bool: Option<bool>,
        #[serde(rename = "Float", default, skip_serializing_if = "Option::is_none")]
        pub float: Option<f64>,
        #[serde(rename = "Int", default, skip_serializing_if = "Option::is_none")]
        pub int: Option<i64>,
        #[serde(rename = "String", default, skip_serializing_if = "Option::is_none")]
        pub string: Option<String>,
        #[serde(rename = "Unit", default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
    }

    impl From<&Attribute> for Attribute {
        fn from(value: &Attribute) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AutopilotConfiguration {
        #[serde(
            rename = "CleanupDeadServers",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub cleanup_dead_servers: Option<bool>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "DisableUpgradeMigration",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub disable_upgrade_migration: Option<bool>,
        #[serde(
            rename = "EnableCustomUpgrades",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_custom_upgrades: Option<bool>,
        #[serde(
            rename = "EnableRedundancyZones",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_redundancy_zones: Option<bool>,
        #[serde(
            rename = "LastContactThreshold",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_contact_threshold: Option<String>,
        #[serde(
            rename = "MaxTrailingLogs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_trailing_logs: Option<u64>,
        #[serde(rename = "MinQuorum", default, skip_serializing_if = "Option::is_none")]
        pub min_quorum: Option<u64>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "ServerStabilizationTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub server_stabilization_time: Option<String>,
    }

    impl From<&AutopilotConfiguration> for AutopilotConfiguration {
        fn from(value: &AutopilotConfiguration) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ChangeScript {
        #[serde(rename = "Args", default, skip_serializing_if = "Vec::is_empty")]
        pub args: Vec<String>,
        #[serde(rename = "Command", default, skip_serializing_if = "Option::is_none")]
        pub command: Option<String>,
        #[serde(
            rename = "FailOnError",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub fail_on_error: Option<bool>,
        #[serde(rename = "Timeout", default, skip_serializing_if = "Option::is_none")]
        pub timeout: Option<i64>,
    }

    impl From<&ChangeScript> for ChangeScript {
        fn from(value: &ChangeScript) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CheckRestart {
        #[serde(rename = "Grace", default, skip_serializing_if = "Option::is_none")]
        pub grace: Option<i64>,
        #[serde(
            rename = "IgnoreWarnings",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ignore_warnings: Option<bool>,
        #[serde(rename = "Limit", default, skip_serializing_if = "Option::is_none")]
        pub limit: Option<i64>,
    }

    impl From<&CheckRestart> for CheckRestart {
        fn from(value: &CheckRestart) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Constraint {
        #[serde(rename = "LTarget", default, skip_serializing_if = "Option::is_none")]
        pub l_target: Option<String>,
        #[serde(rename = "Operand", default, skip_serializing_if = "Option::is_none")]
        pub operand: Option<String>,
        #[serde(rename = "RTarget", default, skip_serializing_if = "Option::is_none")]
        pub r_target: Option<String>,
    }

    impl From<&Constraint> for Constraint {
        fn from(value: &Constraint) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Consul {
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
    }

    impl From<&Consul> for Consul {
        fn from(value: &Consul) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulConnect {
        #[serde(rename = "Gateway", default, skip_serializing_if = "Option::is_none")]
        pub gateway: Option<ConsulGateway>,
        #[serde(rename = "Native", default, skip_serializing_if = "Option::is_none")]
        pub native: Option<bool>,
        #[serde(
            rename = "SidecarService",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sidecar_service: Option<ConsulSidecarService>,
        #[serde(
            rename = "SidecarTask",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sidecar_task: Option<SidecarTask>,
    }

    impl From<&ConsulConnect> for ConsulConnect {
        fn from(value: &ConsulConnect) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulExposeConfig {
        #[serde(rename = "Path", default, skip_serializing_if = "Vec::is_empty")]
        pub path: Vec<ConsulExposePath>,
    }

    impl From<&ConsulExposeConfig> for ConsulExposeConfig {
        fn from(value: &ConsulExposeConfig) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulExposePath {
        #[serde(
            rename = "ListenerPort",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub listener_port: Option<String>,
        #[serde(
            rename = "LocalPathPort",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub local_path_port: Option<i64>,
        #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(rename = "Protocol", default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
    }

    impl From<&ConsulExposePath> for ConsulExposePath {
        fn from(value: &ConsulExposePath) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulGateway {
        #[serde(rename = "Ingress", default, skip_serializing_if = "Option::is_none")]
        pub ingress: Option<ConsulIngressConfigEntry>,
        #[serde(rename = "Mesh", default, skip_serializing_if = "Option::is_none")]
        pub mesh: Option<serde_json::Value>,
        #[serde(rename = "Proxy", default, skip_serializing_if = "Option::is_none")]
        pub proxy: Option<ConsulGatewayProxy>,
        #[serde(
            rename = "Terminating",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub terminating: Option<ConsulTerminatingConfigEntry>,
    }

    impl From<&ConsulGateway> for ConsulGateway {
        fn from(value: &ConsulGateway) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulGatewayBindAddress {
        #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Port", default, skip_serializing_if = "Option::is_none")]
        pub port: Option<i64>,
    }

    impl From<&ConsulGatewayBindAddress> for ConsulGatewayBindAddress {
        fn from(value: &ConsulGatewayBindAddress) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulGatewayProxy {
        #[serde(
            rename = "Config",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub config: std::collections::HashMap<String, serde_json::Value>,
        #[serde(
            rename = "ConnectTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub connect_timeout: Option<i64>,
        #[serde(
            rename = "EnvoyDNSDiscoveryType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub envoy_dns_discovery_type: Option<String>,
        #[serde(
            rename = "EnvoyGatewayBindAddresses",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub envoy_gateway_bind_addresses:
            std::collections::HashMap<String, ConsulGatewayBindAddress>,
        #[serde(
            rename = "EnvoyGatewayBindTaggedAddresses",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub envoy_gateway_bind_tagged_addresses: Option<bool>,
        #[serde(
            rename = "EnvoyGatewayNoDefaultBind",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub envoy_gateway_no_default_bind: Option<bool>,
    }

    impl From<&ConsulGatewayProxy> for ConsulGatewayProxy {
        fn from(value: &ConsulGatewayProxy) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulGatewayTlsConfig {
        #[serde(
            rename = "CipherSuites",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub cipher_suites: Vec<String>,
        #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(
            rename = "TLSMaxVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tls_max_version: Option<String>,
        #[serde(
            rename = "TLSMinVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tls_min_version: Option<String>,
    }

    impl From<&ConsulGatewayTlsConfig> for ConsulGatewayTlsConfig {
        fn from(value: &ConsulGatewayTlsConfig) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulIngressConfigEntry {
        #[serde(rename = "Listeners", default, skip_serializing_if = "Vec::is_empty")]
        pub listeners: Vec<ConsulIngressListener>,
        #[serde(rename = "TLS", default, skip_serializing_if = "Option::is_none")]
        pub tls: Option<ConsulGatewayTlsConfig>,
    }

    impl From<&ConsulIngressConfigEntry> for ConsulIngressConfigEntry {
        fn from(value: &ConsulIngressConfigEntry) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulIngressListener {
        #[serde(rename = "Port", default, skip_serializing_if = "Option::is_none")]
        pub port: Option<i64>,
        #[serde(rename = "Protocol", default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
        #[serde(rename = "Services", default, skip_serializing_if = "Vec::is_empty")]
        pub services: Vec<ConsulIngressService>,
    }

    impl From<&ConsulIngressListener> for ConsulIngressListener {
        fn from(value: &ConsulIngressListener) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulIngressService {
        #[serde(rename = "Hosts", default, skip_serializing_if = "Vec::is_empty")]
        pub hosts: Vec<String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&ConsulIngressService> for ConsulIngressService {
        fn from(value: &ConsulIngressService) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulLinkedService {
        #[serde(rename = "CAFile", default, skip_serializing_if = "Option::is_none")]
        pub ca_file: Option<String>,
        #[serde(rename = "CertFile", default, skip_serializing_if = "Option::is_none")]
        pub cert_file: Option<String>,
        #[serde(rename = "KeyFile", default, skip_serializing_if = "Option::is_none")]
        pub key_file: Option<String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "SNI", default, skip_serializing_if = "Option::is_none")]
        pub sni: Option<String>,
    }

    impl From<&ConsulLinkedService> for ConsulLinkedService {
        fn from(value: &ConsulLinkedService) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulMeshConfigEntry(pub serde_json::Value);
    impl std::ops::Deref for ConsulMeshConfigEntry {
        type Target = serde_json::Value;
        fn deref(&self) -> &serde_json::Value {
            &self.0
        }
    }

    impl From<ConsulMeshConfigEntry> for serde_json::Value {
        fn from(value: ConsulMeshConfigEntry) -> Self {
            value.0
        }
    }

    impl From<&ConsulMeshConfigEntry> for ConsulMeshConfigEntry {
        fn from(value: &ConsulMeshConfigEntry) -> Self {
            value.clone()
        }
    }

    impl From<serde_json::Value> for ConsulMeshConfigEntry {
        fn from(value: serde_json::Value) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulMeshGateway {
        #[serde(rename = "Mode", default, skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,
    }

    impl From<&ConsulMeshGateway> for ConsulMeshGateway {
        fn from(value: &ConsulMeshGateway) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulProxy {
        #[serde(
            rename = "Config",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub config: std::collections::HashMap<String, serde_json::Value>,
        #[serde(
            rename = "ExposeConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub expose_config: Option<ConsulExposeConfig>,
        #[serde(
            rename = "LocalServiceAddress",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub local_service_address: Option<String>,
        #[serde(
            rename = "LocalServicePort",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub local_service_port: Option<i64>,
        #[serde(rename = "Upstreams", default, skip_serializing_if = "Vec::is_empty")]
        pub upstreams: Vec<ConsulUpstream>,
    }

    impl From<&ConsulProxy> for ConsulProxy {
        fn from(value: &ConsulProxy) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulSidecarService {
        #[serde(
            rename = "DisableDefaultTCPCheck",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub disable_default_tcp_check: Option<bool>,
        #[serde(rename = "Port", default, skip_serializing_if = "Option::is_none")]
        pub port: Option<String>,
        #[serde(rename = "Proxy", default, skip_serializing_if = "Option::is_none")]
        pub proxy: Option<ConsulProxy>,
        #[serde(rename = "Tags", default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<String>,
    }

    impl From<&ConsulSidecarService> for ConsulSidecarService {
        fn from(value: &ConsulSidecarService) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulTerminatingConfigEntry {
        #[serde(rename = "Services", default, skip_serializing_if = "Vec::is_empty")]
        pub services: Vec<ConsulLinkedService>,
    }

    impl From<&ConsulTerminatingConfigEntry> for ConsulTerminatingConfigEntry {
        fn from(value: &ConsulTerminatingConfigEntry) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConsulUpstream {
        #[serde(
            rename = "Datacenter",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub datacenter: Option<String>,
        #[serde(
            rename = "DestinationName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub destination_name: Option<String>,
        #[serde(
            rename = "DestinationNamespace",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub destination_namespace: Option<String>,
        #[serde(
            rename = "LocalBindAddress",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub local_bind_address: Option<String>,
        #[serde(
            rename = "LocalBindPort",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub local_bind_port: Option<i64>,
        #[serde(
            rename = "MeshGateway",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mesh_gateway: Option<ConsulMeshGateway>,
    }

    impl From<&ConsulUpstream> for ConsulUpstream {
        fn from(value: &ConsulUpstream) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct Context(pub String);
    impl std::ops::Deref for Context {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Context> for String {
        fn from(value: Context) -> Self {
            value.0
        }
    }

    impl From<&Context> for Context {
        fn from(value: &Context) -> Self {
            value.clone()
        }
    }

    impl From<String> for Context {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Context {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for Context {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiControllerInfo {
        #[serde(
            rename = "SupportsAttachDetach",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_attach_detach: Option<bool>,
        #[serde(
            rename = "SupportsClone",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_clone: Option<bool>,
        #[serde(
            rename = "SupportsCondition",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_condition: Option<bool>,
        #[serde(
            rename = "SupportsCreateDelete",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_create_delete: Option<bool>,
        #[serde(
            rename = "SupportsCreateDeleteSnapshot",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_create_delete_snapshot: Option<bool>,
        #[serde(
            rename = "SupportsExpand",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_expand: Option<bool>,
        #[serde(
            rename = "SupportsGet",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_get: Option<bool>,
        #[serde(
            rename = "SupportsGetCapacity",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_get_capacity: Option<bool>,
        #[serde(
            rename = "SupportsListSnapshots",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_list_snapshots: Option<bool>,
        #[serde(
            rename = "SupportsListVolumes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_list_volumes: Option<bool>,
        #[serde(
            rename = "SupportsListVolumesAttachedNodes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_list_volumes_attached_nodes: Option<bool>,
        #[serde(
            rename = "SupportsReadOnlyAttach",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_read_only_attach: Option<bool>,
    }

    impl From<&CsiControllerInfo> for CsiControllerInfo {
        fn from(value: &CsiControllerInfo) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiInfo {
        #[serde(rename = "AllocID", default, skip_serializing_if = "Option::is_none")]
        pub alloc_id: Option<String>,
        #[serde(
            rename = "ControllerInfo",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controller_info: Option<CsiControllerInfo>,
        #[serde(
            rename = "HealthDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub health_description: Option<String>,
        #[serde(rename = "Healthy", default, skip_serializing_if = "Option::is_none")]
        pub healthy: Option<bool>,
        #[serde(rename = "NodeInfo", default, skip_serializing_if = "Option::is_none")]
        pub node_info: Option<CsiNodeInfo>,
        #[serde(rename = "PluginID", default, skip_serializing_if = "Option::is_none")]
        pub plugin_id: Option<String>,
        #[serde(
            rename = "RequiresControllerPlugin",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub requires_controller_plugin: Option<bool>,
        #[serde(
            rename = "RequiresTopologies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub requires_topologies: Option<bool>,
        #[serde(
            rename = "UpdateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    }

    impl From<&CsiInfo> for CsiInfo {
        fn from(value: &CsiInfo) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiMountOptions {
        #[serde(rename = "FSType", default, skip_serializing_if = "Option::is_none")]
        pub fs_type: Option<String>,
        #[serde(rename = "MountFlags", default, skip_serializing_if = "Vec::is_empty")]
        pub mount_flags: Vec<String>,
    }

    impl From<&CsiMountOptions> for CsiMountOptions {
        fn from(value: &CsiMountOptions) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiNodeInfo {
        #[serde(
            rename = "AccessibleTopology",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub accessible_topology: Option<CsiTopology>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "MaxVolumes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_volumes: Option<i64>,
        #[serde(
            rename = "RequiresNodeStageVolume",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub requires_node_stage_volume: Option<bool>,
        #[serde(
            rename = "SupportsCondition",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_condition: Option<bool>,
        #[serde(
            rename = "SupportsExpand",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_expand: Option<bool>,
        #[serde(
            rename = "SupportsStats",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_stats: Option<bool>,
    }

    impl From<&CsiNodeInfo> for CsiNodeInfo {
        fn from(value: &CsiNodeInfo) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiPlugin {
        #[serde(rename = "Allocations", default, skip_serializing_if = "Vec::is_empty")]
        pub allocations: Vec<AllocationListStub>,
        #[serde(
            rename = "ControllerRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controller_required: Option<bool>,
        #[serde(
            rename = "Controllers",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub controllers: std::collections::HashMap<String, CsiInfo>,
        #[serde(
            rename = "ControllersExpected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controllers_expected: Option<i64>,
        #[serde(
            rename = "ControllersHealthy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controllers_healthy: Option<i64>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "Nodes",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub nodes: std::collections::HashMap<String, CsiInfo>,
        #[serde(
            rename = "NodesExpected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_expected: Option<i64>,
        #[serde(
            rename = "NodesHealthy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_healthy: Option<i64>,
        #[serde(rename = "Provider", default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }

    impl From<&CsiPlugin> for CsiPlugin {
        fn from(value: &CsiPlugin) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiPluginListStub {
        #[serde(
            rename = "ControllerRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controller_required: Option<bool>,
        #[serde(
            rename = "ControllersExpected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controllers_expected: Option<i64>,
        #[serde(
            rename = "ControllersHealthy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controllers_healthy: Option<i64>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "NodesExpected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_expected: Option<i64>,
        #[serde(
            rename = "NodesHealthy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_healthy: Option<i64>,
        #[serde(rename = "Provider", default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
    }

    impl From<&CsiPluginListStub> for CsiPluginListStub {
        fn from(value: &CsiPluginListStub) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct CsiPluginType(pub String);
    impl std::ops::Deref for CsiPluginType {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<CsiPluginType> for String {
        fn from(value: CsiPluginType) -> Self {
            value.0
        }
    }

    impl From<&CsiPluginType> for CsiPluginType {
        fn from(value: &CsiPluginType) -> Self {
            value.clone()
        }
    }

    impl From<String> for CsiPluginType {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for CsiPluginType {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for CsiPluginType {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiSecrets(pub std::collections::HashMap<String, String>);
    impl std::ops::Deref for CsiSecrets {
        type Target = std::collections::HashMap<String, String>;
        fn deref(&self) -> &std::collections::HashMap<String, String> {
            &self.0
        }
    }

    impl From<CsiSecrets> for std::collections::HashMap<String, String> {
        fn from(value: CsiSecrets) -> Self {
            value.0
        }
    }

    impl From<&CsiSecrets> for CsiSecrets {
        fn from(value: &CsiSecrets) -> Self {
            value.clone()
        }
    }

    impl From<std::collections::HashMap<String, String>> for CsiSecrets {
        fn from(value: std::collections::HashMap<String, String>) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiSnapshot {
        #[serde(
            rename = "CreateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_time: Option<i64>,
        #[serde(
            rename = "ExternalSourceVolumeID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub external_source_volume_id: Option<String>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "IsReady", default, skip_serializing_if = "Option::is_none")]
        pub is_ready: Option<bool>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "Parameters",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub parameters: std::collections::HashMap<String, String>,
        #[serde(rename = "PluginID", default, skip_serializing_if = "Option::is_none")]
        pub plugin_id: Option<String>,
        #[serde(rename = "Secrets", default, skip_serializing_if = "Option::is_none")]
        pub secrets: Option<CsiSecrets>,
        #[serde(rename = "SizeBytes", default, skip_serializing_if = "Option::is_none")]
        pub size_bytes: Option<i64>,
        #[serde(
            rename = "SourceVolumeID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub source_volume_id: Option<String>,
    }

    impl From<&CsiSnapshot> for CsiSnapshot {
        fn from(value: &CsiSnapshot) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiSnapshotCreateRequest {
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
        #[serde(rename = "Snapshots", default, skip_serializing_if = "Vec::is_empty")]
        pub snapshots: Vec<CsiSnapshot>,
    }

    impl From<&CsiSnapshotCreateRequest> for CsiSnapshotCreateRequest {
        fn from(value: &CsiSnapshotCreateRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiSnapshotCreateResponse {
        #[serde(
            rename = "KnownLeader",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub known_leader: Option<bool>,
        #[serde(
            rename = "LastContact",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_contact: Option<i64>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
        #[serde(rename = "Snapshots", default, skip_serializing_if = "Vec::is_empty")]
        pub snapshots: Vec<CsiSnapshot>,
    }

    impl From<&CsiSnapshotCreateResponse> for CsiSnapshotCreateResponse {
        fn from(value: &CsiSnapshotCreateResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiSnapshotListResponse {
        #[serde(
            rename = "KnownLeader",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub known_leader: Option<bool>,
        #[serde(
            rename = "LastContact",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_contact: Option<i64>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
        #[serde(rename = "Snapshots", default, skip_serializing_if = "Vec::is_empty")]
        pub snapshots: Vec<CsiSnapshot>,
    }

    impl From<&CsiSnapshotListResponse> for CsiSnapshotListResponse {
        fn from(value: &CsiSnapshotListResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiTopology {
        #[serde(
            rename = "Segments",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub segments: std::collections::HashMap<String, String>,
    }

    impl From<&CsiTopology> for CsiTopology {
        fn from(value: &CsiTopology) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiTopologyRequest {
        #[serde(rename = "Preferred", default, skip_serializing_if = "Vec::is_empty")]
        pub preferred: Vec<CsiTopology>,
        #[serde(rename = "Required", default, skip_serializing_if = "Vec::is_empty")]
        pub required: Vec<CsiTopology>,
    }

    impl From<&CsiTopologyRequest> for CsiTopologyRequest {
        fn from(value: &CsiTopologyRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiVolume {
        #[serde(
            rename = "AccessMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_mode: Option<String>,
        #[serde(rename = "Allocations", default, skip_serializing_if = "Vec::is_empty")]
        pub allocations: Vec<AllocationListStub>,
        #[serde(
            rename = "AttachmentMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attachment_mode: Option<String>,
        #[serde(rename = "Capacity", default, skip_serializing_if = "Option::is_none")]
        pub capacity: Option<i64>,
        #[serde(rename = "CloneID", default, skip_serializing_if = "Option::is_none")]
        pub clone_id: Option<String>,
        #[serde(
            rename = "Context",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub context: std::collections::HashMap<String, String>,
        #[serde(
            rename = "ControllerRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controller_required: Option<bool>,
        #[serde(
            rename = "ControllersExpected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controllers_expected: Option<i64>,
        #[serde(
            rename = "ControllersHealthy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controllers_healthy: Option<i64>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "ExternalID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub external_id: Option<String>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "MountOptions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mount_options: Option<CsiMountOptions>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "NodesExpected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_expected: Option<i64>,
        #[serde(
            rename = "NodesHealthy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_healthy: Option<i64>,
        #[serde(
            rename = "Parameters",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub parameters: std::collections::HashMap<String, String>,
        #[serde(rename = "PluginID", default, skip_serializing_if = "Option::is_none")]
        pub plugin_id: Option<String>,
        #[serde(rename = "Provider", default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(
            rename = "ProviderVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_version: Option<String>,
        #[serde(
            rename = "ReadAllocs",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub read_allocs: std::collections::HashMap<String, Allocation>,
        #[serde(
            rename = "RequestedCapabilities",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub requested_capabilities: Vec<CsiVolumeCapability>,
        #[serde(
            rename = "RequestedCapacityMax",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub requested_capacity_max: Option<i64>,
        #[serde(
            rename = "RequestedCapacityMin",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub requested_capacity_min: Option<i64>,
        #[serde(
            rename = "RequestedTopologies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub requested_topologies: Option<CsiTopologyRequest>,
        #[serde(
            rename = "ResourceExhausted",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub resource_exhausted: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(
            rename = "Schedulable",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub schedulable: Option<bool>,
        #[serde(rename = "Secrets", default, skip_serializing_if = "Option::is_none")]
        pub secrets: Option<CsiSecrets>,
        #[serde(
            rename = "SnapshotID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub snapshot_id: Option<String>,
        #[serde(rename = "Topologies", default, skip_serializing_if = "Vec::is_empty")]
        pub topologies: Vec<CsiTopology>,
        #[serde(
            rename = "WriteAllocs",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub write_allocs: std::collections::HashMap<String, Allocation>,
    }

    impl From<&CsiVolume> for CsiVolume {
        fn from(value: &CsiVolume) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct CsiVolumeAccessMode(pub String);
    impl std::ops::Deref for CsiVolumeAccessMode {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<CsiVolumeAccessMode> for String {
        fn from(value: CsiVolumeAccessMode) -> Self {
            value.0
        }
    }

    impl From<&CsiVolumeAccessMode> for CsiVolumeAccessMode {
        fn from(value: &CsiVolumeAccessMode) -> Self {
            value.clone()
        }
    }

    impl From<String> for CsiVolumeAccessMode {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for CsiVolumeAccessMode {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for CsiVolumeAccessMode {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct CsiVolumeAttachmentMode(pub String);
    impl std::ops::Deref for CsiVolumeAttachmentMode {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<CsiVolumeAttachmentMode> for String {
        fn from(value: CsiVolumeAttachmentMode) -> Self {
            value.0
        }
    }

    impl From<&CsiVolumeAttachmentMode> for CsiVolumeAttachmentMode {
        fn from(value: &CsiVolumeAttachmentMode) -> Self {
            value.clone()
        }
    }

    impl From<String> for CsiVolumeAttachmentMode {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for CsiVolumeAttachmentMode {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for CsiVolumeAttachmentMode {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiVolumeCapability {
        #[serde(
            rename = "AccessMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_mode: Option<String>,
        #[serde(
            rename = "AttachmentMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attachment_mode: Option<String>,
    }

    impl From<&CsiVolumeCapability> for CsiVolumeCapability {
        fn from(value: &CsiVolumeCapability) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiVolumeCreateRequest {
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
        #[serde(rename = "Volumes", default, skip_serializing_if = "Vec::is_empty")]
        pub volumes: Vec<CsiVolume>,
    }

    impl From<&CsiVolumeCreateRequest> for CsiVolumeCreateRequest {
        fn from(value: &CsiVolumeCreateRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiVolumeExternalStub {
        #[serde(
            rename = "CapacityBytes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub capacity_bytes: Option<i64>,
        #[serde(rename = "CloneID", default, skip_serializing_if = "Option::is_none")]
        pub clone_id: Option<String>,
        #[serde(
            rename = "ExternalID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub external_id: Option<String>,
        #[serde(
            rename = "IsAbnormal",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub is_abnormal: Option<bool>,
        #[serde(
            rename = "PublishedExternalNodeIDs",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub published_external_node_i_ds: Vec<String>,
        #[serde(
            rename = "SnapshotID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub snapshot_id: Option<String>,
        #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(
            rename = "VolumeContext",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub volume_context: std::collections::HashMap<String, String>,
    }

    impl From<&CsiVolumeExternalStub> for CsiVolumeExternalStub {
        fn from(value: &CsiVolumeExternalStub) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiVolumeListExternalResponse {
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(rename = "Volumes", default, skip_serializing_if = "Vec::is_empty")]
        pub volumes: Vec<CsiVolumeExternalStub>,
    }

    impl From<&CsiVolumeListExternalResponse> for CsiVolumeListExternalResponse {
        fn from(value: &CsiVolumeListExternalResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiVolumeListStub {
        #[serde(
            rename = "AccessMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_mode: Option<String>,
        #[serde(
            rename = "AttachmentMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attachment_mode: Option<String>,
        #[serde(
            rename = "ControllerRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controller_required: Option<bool>,
        #[serde(
            rename = "ControllersExpected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controllers_expected: Option<i64>,
        #[serde(
            rename = "ControllersHealthy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controllers_healthy: Option<i64>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "CurrentReaders",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub current_readers: Option<i64>,
        #[serde(
            rename = "CurrentWriters",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub current_writers: Option<i64>,
        #[serde(
            rename = "ExternalID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub external_id: Option<String>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "NodesExpected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_expected: Option<i64>,
        #[serde(
            rename = "NodesHealthy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nodes_healthy: Option<i64>,
        #[serde(rename = "PluginID", default, skip_serializing_if = "Option::is_none")]
        pub plugin_id: Option<String>,
        #[serde(rename = "Provider", default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(
            rename = "ResourceExhausted",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub resource_exhausted: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(
            rename = "Schedulable",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub schedulable: Option<bool>,
        #[serde(rename = "Topologies", default, skip_serializing_if = "Vec::is_empty")]
        pub topologies: Vec<CsiTopology>,
    }

    impl From<&CsiVolumeListStub> for CsiVolumeListStub {
        fn from(value: &CsiVolumeListStub) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CsiVolumeRegisterRequest {
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
        #[serde(rename = "Volumes", default, skip_serializing_if = "Vec::is_empty")]
        pub volumes: Vec<CsiVolume>,
    }

    impl From<&CsiVolumeRegisterRequest> for CsiVolumeRegisterRequest {
        fn from(value: &CsiVolumeRegisterRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Deployment {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "IsMultiregion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub is_multiregion: Option<bool>,
        #[serde(
            rename = "JobCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_create_index: Option<u64>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(
            rename = "JobModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_modify_index: Option<u64>,
        #[serde(
            rename = "JobSpecModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_spec_modify_index: Option<u64>,
        #[serde(
            rename = "JobVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_version: Option<u64>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(
            rename = "StatusDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub status_description: Option<String>,
        #[serde(
            rename = "TaskGroups",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub task_groups: std::collections::HashMap<String, DeploymentState>,
    }

    impl From<&Deployment> for Deployment {
        fn from(value: &Deployment) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeploymentAllocHealthRequest {
        #[serde(
            rename = "DeploymentID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deployment_id: Option<String>,
        #[serde(
            rename = "HealthyAllocationIDs",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub healthy_allocation_i_ds: Vec<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
        #[serde(
            rename = "UnhealthyAllocationIDs",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub unhealthy_allocation_i_ds: Vec<String>,
    }

    impl From<&DeploymentAllocHealthRequest> for DeploymentAllocHealthRequest {
        fn from(value: &DeploymentAllocHealthRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeploymentPauseRequest {
        #[serde(
            rename = "DeploymentID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deployment_id: Option<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Pause", default, skip_serializing_if = "Option::is_none")]
        pub pause: Option<bool>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
    }

    impl From<&DeploymentPauseRequest> for DeploymentPauseRequest {
        fn from(value: &DeploymentPauseRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeploymentPromoteRequest {
        #[serde(rename = "All", default, skip_serializing_if = "Option::is_none")]
        pub all: Option<bool>,
        #[serde(
            rename = "DeploymentID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deployment_id: Option<String>,
        #[serde(rename = "Groups", default, skip_serializing_if = "Vec::is_empty")]
        pub groups: Vec<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
    }

    impl From<&DeploymentPromoteRequest> for DeploymentPromoteRequest {
        fn from(value: &DeploymentPromoteRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeploymentState {
        #[serde(
            rename = "AutoRevert",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub auto_revert: Option<bool>,
        #[serde(
            rename = "DesiredCanaries",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub desired_canaries: Option<i64>,
        #[serde(
            rename = "DesiredTotal",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub desired_total: Option<i64>,
        #[serde(
            rename = "HealthyAllocs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub healthy_allocs: Option<i64>,
        #[serde(
            rename = "PlacedAllocs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub placed_allocs: Option<i64>,
        #[serde(
            rename = "PlacedCanaries",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub placed_canaries: Vec<String>,
        #[serde(
            rename = "ProgressDeadline",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub progress_deadline: Option<i64>,
        #[serde(rename = "Promoted", default, skip_serializing_if = "Option::is_none")]
        pub promoted: Option<bool>,
        #[serde(
            rename = "RequireProgressBy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub require_progress_by: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(
            rename = "UnhealthyAllocs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub unhealthy_allocs: Option<i64>,
    }

    impl From<&DeploymentState> for DeploymentState {
        fn from(value: &DeploymentState) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeploymentUnblockRequest {
        #[serde(
            rename = "DeploymentID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deployment_id: Option<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
    }

    impl From<&DeploymentUnblockRequest> for DeploymentUnblockRequest {
        fn from(value: &DeploymentUnblockRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeploymentUpdateResponse {
        #[serde(
            rename = "DeploymentModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deployment_modify_index: Option<u64>,
        #[serde(
            rename = "EvalCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eval_create_index: Option<u64>,
        #[serde(rename = "EvalID", default, skip_serializing_if = "Option::is_none")]
        pub eval_id: Option<String>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
        #[serde(
            rename = "RevertedJobVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reverted_job_version: Option<u64>,
    }

    impl From<&DeploymentUpdateResponse> for DeploymentUpdateResponse {
        fn from(value: &DeploymentUpdateResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DesiredTransition {
        #[serde(rename = "Migrate", default, skip_serializing_if = "Option::is_none")]
        pub migrate: Option<bool>,
        #[serde(
            rename = "Reschedule",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reschedule: Option<bool>,
    }

    impl From<&DesiredTransition> for DesiredTransition {
        fn from(value: &DesiredTransition) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DesiredUpdates {
        #[serde(rename = "Canary", default, skip_serializing_if = "Option::is_none")]
        pub canary: Option<u64>,
        #[serde(
            rename = "DestructiveUpdate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub destructive_update: Option<u64>,
        #[serde(rename = "Ignore", default, skip_serializing_if = "Option::is_none")]
        pub ignore: Option<u64>,
        #[serde(
            rename = "InPlaceUpdate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub in_place_update: Option<u64>,
        #[serde(rename = "Migrate", default, skip_serializing_if = "Option::is_none")]
        pub migrate: Option<u64>,
        #[serde(rename = "Place", default, skip_serializing_if = "Option::is_none")]
        pub place: Option<u64>,
        #[serde(
            rename = "Preemptions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub preemptions: Option<u64>,
        #[serde(rename = "Stop", default, skip_serializing_if = "Option::is_none")]
        pub stop: Option<u64>,
    }

    impl From<&DesiredUpdates> for DesiredUpdates {
        fn from(value: &DesiredUpdates) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DispatchPayloadConfig {
        #[serde(rename = "File", default, skip_serializing_if = "Option::is_none")]
        pub file: Option<String>,
    }

    impl From<&DispatchPayloadConfig> for DispatchPayloadConfig {
        fn from(value: &DispatchPayloadConfig) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DnsConfig {
        #[serde(rename = "Options", default, skip_serializing_if = "Vec::is_empty")]
        pub options: Vec<String>,
        #[serde(rename = "Searches", default, skip_serializing_if = "Vec::is_empty")]
        pub searches: Vec<String>,
        #[serde(rename = "Servers", default, skip_serializing_if = "Vec::is_empty")]
        pub servers: Vec<String>,
    }

    impl From<&DnsConfig> for DnsConfig {
        fn from(value: &DnsConfig) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DrainMetadata {
        #[serde(
            rename = "AccessorID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub accessor_id: Option<String>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(rename = "StartedAt", default, skip_serializing_if = "Option::is_none")]
        pub started_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(rename = "UpdatedAt", default, skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    }

    impl From<&DrainMetadata> for DrainMetadata {
        fn from(value: &DrainMetadata) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DrainSpec {
        #[serde(rename = "Deadline", default, skip_serializing_if = "Option::is_none")]
        pub deadline: Option<i64>,
        #[serde(
            rename = "IgnoreSystemJobs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ignore_system_jobs: Option<bool>,
    }

    impl From<&DrainSpec> for DrainSpec {
        fn from(value: &DrainSpec) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct DrainStatus(pub String);
    impl std::ops::Deref for DrainStatus {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<DrainStatus> for String {
        fn from(value: DrainStatus) -> Self {
            value.0
        }
    }

    impl From<&DrainStatus> for DrainStatus {
        fn from(value: &DrainStatus) -> Self {
            value.clone()
        }
    }

    impl From<String> for DrainStatus {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for DrainStatus {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for DrainStatus {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DrainStrategy {
        #[serde(rename = "Deadline", default, skip_serializing_if = "Option::is_none")]
        pub deadline: Option<i64>,
        #[serde(
            rename = "ForceDeadline",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub force_deadline: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(
            rename = "IgnoreSystemJobs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ignore_system_jobs: Option<bool>,
        #[serde(rename = "StartedAt", default, skip_serializing_if = "Option::is_none")]
        pub started_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    }

    impl From<&DrainStrategy> for DrainStrategy {
        fn from(value: &DrainStrategy) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DriverInfo {
        #[serde(
            rename = "Attributes",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub attributes: std::collections::HashMap<String, String>,
        #[serde(rename = "Detected", default, skip_serializing_if = "Option::is_none")]
        pub detected: Option<bool>,
        #[serde(
            rename = "HealthDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub health_description: Option<String>,
        #[serde(rename = "Healthy", default, skip_serializing_if = "Option::is_none")]
        pub healthy: Option<bool>,
        #[serde(
            rename = "UpdateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    }

    impl From<&DriverInfo> for DriverInfo {
        fn from(value: &DriverInfo) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Duration(pub i64);
    impl std::ops::Deref for Duration {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl From<Duration> for i64 {
        fn from(value: Duration) -> Self {
            value.0
        }
    }

    impl From<&Duration> for Duration {
        fn from(value: &Duration) -> Self {
            value.clone()
        }
    }

    impl From<i64> for Duration {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Duration {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Duration {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Duration {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Duration {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Duration {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EphemeralDisk {
        #[serde(rename = "Migrate", default, skip_serializing_if = "Option::is_none")]
        pub migrate: Option<bool>,
        #[serde(rename = "SizeMB", default, skip_serializing_if = "Option::is_none")]
        pub size_mb: Option<i64>,
        #[serde(rename = "Sticky", default, skip_serializing_if = "Option::is_none")]
        pub sticky: Option<bool>,
    }

    impl From<&EphemeralDisk> for EphemeralDisk {
        fn from(value: &EphemeralDisk) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EvalOptions {
        #[serde(
            rename = "ForceReschedule",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub force_reschedule: Option<bool>,
    }

    impl From<&EvalOptions> for EvalOptions {
        fn from(value: &EvalOptions) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Evaluation {
        #[serde(
            rename = "AnnotatePlan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub annotate_plan: Option<bool>,
        #[serde(
            rename = "BlockedEval",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blocked_eval: Option<String>,
        #[serde(
            rename = "ClassEligibility",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub class_eligibility: std::collections::HashMap<String, bool>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "CreateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_time: Option<i64>,
        #[serde(
            rename = "DeploymentID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deployment_id: Option<String>,
        #[serde(
            rename = "EscapedComputedClass",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub escaped_computed_class: Option<bool>,
        #[serde(
            rename = "FailedTGAllocs",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub failed_tg_allocs: std::collections::HashMap<String, AllocationMetric>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(
            rename = "JobModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_modify_index: Option<u64>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "ModifyTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_time: Option<i64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "NextEval", default, skip_serializing_if = "Option::is_none")]
        pub next_eval: Option<String>,
        #[serde(rename = "NodeID", default, skip_serializing_if = "Option::is_none")]
        pub node_id: Option<String>,
        #[serde(
            rename = "NodeModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub node_modify_index: Option<u64>,
        #[serde(
            rename = "PreviousEval",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub previous_eval: Option<String>,
        #[serde(rename = "Priority", default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i64>,
        #[serde(
            rename = "QueuedAllocations",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub queued_allocations: std::collections::HashMap<String, i64>,
        #[serde(
            rename = "QuotaLimitReached",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub quota_limit_reached: Option<String>,
        #[serde(
            rename = "RelatedEvals",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub related_evals: Vec<EvaluationStub>,
        #[serde(
            rename = "SnapshotIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub snapshot_index: Option<u64>,
        #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(
            rename = "StatusDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub status_description: Option<String>,
        #[serde(
            rename = "TriggeredBy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub triggered_by: Option<String>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(rename = "Wait", default, skip_serializing_if = "Option::is_none")]
        pub wait: Option<i64>,
        #[serde(rename = "WaitUntil", default, skip_serializing_if = "Option::is_none")]
        pub wait_until: Option<chrono::DateTime<chrono::offset::Utc>>,
    }

    impl From<&Evaluation> for Evaluation {
        fn from(value: &Evaluation) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EvaluationStub {
        #[serde(
            rename = "BlockedEval",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blocked_eval: Option<String>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "CreateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_time: Option<i64>,
        #[serde(
            rename = "DeploymentID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub deployment_id: Option<String>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "ModifyTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_time: Option<i64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "NextEval", default, skip_serializing_if = "Option::is_none")]
        pub next_eval: Option<String>,
        #[serde(rename = "NodeID", default, skip_serializing_if = "Option::is_none")]
        pub node_id: Option<String>,
        #[serde(
            rename = "PreviousEval",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub previous_eval: Option<String>,
        #[serde(rename = "Priority", default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i64>,
        #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(
            rename = "StatusDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub status_description: Option<String>,
        #[serde(
            rename = "TriggeredBy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub triggered_by: Option<String>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(rename = "WaitUntil", default, skip_serializing_if = "Option::is_none")]
        pub wait_until: Option<chrono::DateTime<chrono::offset::Utc>>,
    }

    impl From<&EvaluationStub> for EvaluationStub {
        fn from(value: &EvaluationStub) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FieldDiff {
        #[serde(rename = "Annotations", default, skip_serializing_if = "Vec::is_empty")]
        pub annotations: Vec<String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "New", default, skip_serializing_if = "Option::is_none")]
        pub new: Option<String>,
        #[serde(rename = "Old", default, skip_serializing_if = "Option::is_none")]
        pub old: Option<String>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&FieldDiff> for FieldDiff {
        fn from(value: &FieldDiff) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Float32(pub f64);
    impl std::ops::Deref for Float32 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<Float32> for f64 {
        fn from(value: Float32) -> Self {
            value.0
        }
    }

    impl From<&Float32> for Float32 {
        fn from(value: &Float32) -> Self {
            value.clone()
        }
    }

    impl From<f64> for Float32 {
        fn from(value: f64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Float32 {
        type Err = <f64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Float32 {
        type Error = <f64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Float32 {
        type Error = <f64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Float32 {
        type Error = <f64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Float32 {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Float64(pub f64);
    impl std::ops::Deref for Float64 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<Float64> for f64 {
        fn from(value: Float64) -> Self {
            value.0
        }
    }

    impl From<&Float64> for Float64 {
        fn from(value: &Float64) -> Self {
            value.clone()
        }
    }

    impl From<f64> for Float64 {
        fn from(value: f64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Float64 {
        type Err = <f64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Float64 {
        type Error = <f64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Float64 {
        type Error = <f64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Float64 {
        type Error = <f64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Float64 {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FuzzyMatch {
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "Scope", default, skip_serializing_if = "Vec::is_empty")]
        pub scope: Vec<String>,
    }

    impl From<&FuzzyMatch> for FuzzyMatch {
        fn from(value: &FuzzyMatch) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FuzzySearchRequest {
        #[serde(
            rename = "AllowStale",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_stale: Option<bool>,
        #[serde(rename = "AuthToken", default, skip_serializing_if = "Option::is_none")]
        pub auth_token: Option<String>,
        #[serde(rename = "Context", default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[serde(rename = "Filter", default, skip_serializing_if = "Option::is_none")]
        pub filter: Option<String>,
        #[serde(
            rename = "Headers",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub headers: std::collections::HashMap<String, String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(
            rename = "Params",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub params: std::collections::HashMap<String, String>,
        #[serde(rename = "PerPage", default, skip_serializing_if = "Option::is_none")]
        pub per_page: Option<i32>,
        #[serde(rename = "Prefix", default, skip_serializing_if = "Option::is_none")]
        pub prefix: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "Reverse", default, skip_serializing_if = "Option::is_none")]
        pub reverse: Option<bool>,
        #[serde(rename = "Text", default, skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,
        #[serde(rename = "WaitIndex", default, skip_serializing_if = "Option::is_none")]
        pub wait_index: Option<u64>,
        #[serde(rename = "WaitTime", default, skip_serializing_if = "Option::is_none")]
        pub wait_time: Option<i64>,
    }

    impl From<&FuzzySearchRequest> for FuzzySearchRequest {
        fn from(value: &FuzzySearchRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FuzzySearchResponse {
        #[serde(
            rename = "KnownLeader",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub known_leader: Option<bool>,
        #[serde(
            rename = "LastContact",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_contact: Option<i64>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(
            rename = "Matches",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub matches: std::collections::HashMap<String, Vec<FuzzyMatch>>,
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
        #[serde(
            rename = "Truncations",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub truncations: std::collections::HashMap<String, bool>,
    }

    impl From<&FuzzySearchResponse> for FuzzySearchResponse {
        fn from(value: &FuzzySearchResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GaugeValue {
        #[serde(
            rename = "Labels",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub labels: std::collections::HashMap<String, String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
        pub value: Option<f64>,
    }

    impl From<&GaugeValue> for GaugeValue {
        fn from(value: &GaugeValue) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct HostNetworkInfo {
        #[serde(rename = "CIDR", default, skip_serializing_if = "Option::is_none")]
        pub cidr: Option<String>,
        #[serde(rename = "Interface", default, skip_serializing_if = "Option::is_none")]
        pub interface: Option<String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "ReservedPorts",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reserved_ports: Option<String>,
    }

    impl From<&HostNetworkInfo> for HostNetworkInfo {
        fn from(value: &HostNetworkInfo) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct HostVolumeInfo {
        #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(rename = "ReadOnly", default, skip_serializing_if = "Option::is_none")]
        pub read_only: Option<bool>,
    }

    impl From<&HostVolumeInfo> for HostVolumeInfo {
        fn from(value: &HostVolumeInfo) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Int(pub i64);
    impl std::ops::Deref for Int {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl From<Int> for i64 {
        fn from(value: Int) -> Self {
            value.0
        }
    }

    impl From<&Int> for Int {
        fn from(value: &Int) -> Self {
            value.clone()
        }
    }

    impl From<i64> for Int {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Int {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Int {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Int {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Int {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Int {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Int32(pub i32);
    impl std::ops::Deref for Int32 {
        type Target = i32;
        fn deref(&self) -> &i32 {
            &self.0
        }
    }

    impl From<Int32> for i32 {
        fn from(value: Int32) -> Self {
            value.0
        }
    }

    impl From<&Int32> for Int32 {
        fn from(value: &Int32) -> Self {
            value.clone()
        }
    }

    impl From<i32> for Int32 {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Int32 {
        type Err = <i32 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Int32 {
        type Error = <i32 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Int32 {
        type Error = <i32 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Int32 {
        type Error = <i32 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Int32 {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Int64(pub i64);
    impl std::ops::Deref for Int64 {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl From<Int64> for i64 {
        fn from(value: Int64) -> Self {
            value.0
        }
    }

    impl From<&Int64> for Int64 {
        fn from(value: &Int64) -> Self {
            value.clone()
        }
    }

    impl From<i64> for Int64 {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Int64 {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Int64 {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Int64 {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Int64 {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Int64 {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Int8(pub i8);
    impl std::ops::Deref for Int8 {
        type Target = i8;
        fn deref(&self) -> &i8 {
            &self.0
        }
    }

    impl From<Int8> for i8 {
        fn from(value: Int8) -> Self {
            value.0
        }
    }

    impl From<&Int8> for Int8 {
        fn from(value: &Int8) -> Self {
            value.clone()
        }
    }

    impl From<i8> for Int8 {
        fn from(value: i8) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Int8 {
        type Err = <i8 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Int8 {
        type Error = <i8 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Int8 {
        type Error = <i8 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Int8 {
        type Error = <i8 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Int8 {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Job {
        #[serde(rename = "Affinities", default, skip_serializing_if = "Vec::is_empty")]
        pub affinities: Vec<Affinity>,
        #[serde(rename = "AllAtOnce", default, skip_serializing_if = "Option::is_none")]
        pub all_at_once: Option<bool>,
        #[serde(rename = "Constraints", default, skip_serializing_if = "Vec::is_empty")]
        pub constraints: Vec<Constraint>,
        #[serde(
            rename = "ConsulNamespace",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub consul_namespace: Option<String>,
        #[serde(
            rename = "ConsulToken",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub consul_token: Option<String>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(rename = "Datacenters", default, skip_serializing_if = "Vec::is_empty")]
        pub datacenters: Vec<String>,
        #[serde(
            rename = "DispatchIdempotencyToken",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub dispatch_idempotency_token: Option<String>,
        #[serde(
            rename = "Dispatched",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub dispatched: Option<bool>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "JobModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_modify_index: Option<u64>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(rename = "Migrate", default, skip_serializing_if = "Option::is_none")]
        pub migrate: Option<MigrateStrategy>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "Multiregion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub multiregion: Option<Multiregion>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "NomadTokenID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub nomad_token_id: Option<String>,
        #[serde(
            rename = "ParameterizedJob",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub parameterized_job: Option<ParameterizedJobConfig>,
        #[serde(rename = "ParentID", default, skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        #[serde(rename = "Payload", default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<String>,
        #[serde(rename = "Periodic", default, skip_serializing_if = "Option::is_none")]
        pub periodic: Option<PeriodicConfig>,
        #[serde(rename = "Priority", default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i64>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(
            rename = "Reschedule",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reschedule: Option<ReschedulePolicy>,
        #[serde(rename = "Spreads", default, skip_serializing_if = "Vec::is_empty")]
        pub spreads: Vec<Spread>,
        #[serde(rename = "Stable", default, skip_serializing_if = "Option::is_none")]
        pub stable: Option<bool>,
        #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(
            rename = "StatusDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub status_description: Option<String>,
        #[serde(rename = "Stop", default, skip_serializing_if = "Option::is_none")]
        pub stop: Option<bool>,
        #[serde(
            rename = "SubmitTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub submit_time: Option<i64>,
        #[serde(rename = "TaskGroups", default, skip_serializing_if = "Vec::is_empty")]
        pub task_groups: Vec<TaskGroup>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(rename = "Update", default, skip_serializing_if = "Option::is_none")]
        pub update: Option<UpdateStrategy>,
        #[serde(
            rename = "VaultNamespace",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub vault_namespace: Option<String>,
        #[serde(
            rename = "VaultToken",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub vault_token: Option<String>,
        #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
        pub version: Option<u64>,
    }

    impl From<&Job> for Job {
        fn from(value: &Job) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobAcl {
        #[serde(rename = "Group", default, skip_serializing_if = "Option::is_none")]
        pub group: Option<String>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Task", default, skip_serializing_if = "Option::is_none")]
        pub task: Option<String>,
    }

    impl From<&JobAcl> for JobAcl {
        fn from(value: &JobAcl) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobChildrenSummary {
        #[serde(rename = "Dead", default, skip_serializing_if = "Option::is_none")]
        pub dead: Option<i64>,
        #[serde(rename = "Pending", default, skip_serializing_if = "Option::is_none")]
        pub pending: Option<i64>,
        #[serde(rename = "Running", default, skip_serializing_if = "Option::is_none")]
        pub running: Option<i64>,
    }

    impl From<&JobChildrenSummary> for JobChildrenSummary {
        fn from(value: &JobChildrenSummary) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobDeregisterResponse {
        #[serde(
            rename = "EvalCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eval_create_index: Option<u64>,
        #[serde(rename = "EvalID", default, skip_serializing_if = "Option::is_none")]
        pub eval_id: Option<String>,
        #[serde(
            rename = "JobModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_modify_index: Option<u64>,
        #[serde(
            rename = "KnownLeader",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub known_leader: Option<bool>,
        #[serde(
            rename = "LastContact",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_contact: Option<i64>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
    }

    impl From<&JobDeregisterResponse> for JobDeregisterResponse {
        fn from(value: &JobDeregisterResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobDiff {
        #[serde(rename = "Fields", default, skip_serializing_if = "Vec::is_empty")]
        pub fields: Vec<FieldDiff>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "Objects", default, skip_serializing_if = "Vec::is_empty")]
        pub objects: Vec<ObjectDiff>,
        #[serde(rename = "TaskGroups", default, skip_serializing_if = "Vec::is_empty")]
        pub task_groups: Vec<TaskGroupDiff>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&JobDiff> for JobDiff {
        fn from(value: &JobDiff) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobDispatchRequest {
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(rename = "Payload", default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<String>,
    }

    impl From<&JobDispatchRequest> for JobDispatchRequest {
        fn from(value: &JobDispatchRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobDispatchResponse {
        #[serde(
            rename = "DispatchedJobID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub dispatched_job_id: Option<String>,
        #[serde(
            rename = "EvalCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eval_create_index: Option<u64>,
        #[serde(rename = "EvalID", default, skip_serializing_if = "Option::is_none")]
        pub eval_id: Option<String>,
        #[serde(
            rename = "JobCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_create_index: Option<u64>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
    }

    impl From<&JobDispatchResponse> for JobDispatchResponse {
        fn from(value: &JobDispatchResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobEvaluateRequest {
        #[serde(
            rename = "EvalOptions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eval_options: Option<EvalOptions>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
    }

    impl From<&JobEvaluateRequest> for JobEvaluateRequest {
        fn from(value: &JobEvaluateRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobListStub {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(rename = "Datacenters", default, skip_serializing_if = "Vec::is_empty")]
        pub datacenters: Vec<String>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "JobModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_modify_index: Option<u64>,
        #[serde(
            rename = "JobSummary",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_summary: Option<JobSummary>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "ParameterizedJob",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub parameterized_job: Option<bool>,
        #[serde(rename = "ParentID", default, skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        #[serde(rename = "Periodic", default, skip_serializing_if = "Option::is_none")]
        pub periodic: Option<bool>,
        #[serde(rename = "Priority", default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i64>,
        #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(
            rename = "StatusDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub status_description: Option<String>,
        #[serde(rename = "Stop", default, skip_serializing_if = "Option::is_none")]
        pub stop: Option<bool>,
        #[serde(
            rename = "SubmitTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub submit_time: Option<i64>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&JobListStub> for JobListStub {
        fn from(value: &JobListStub) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobPlanRequest {
        #[serde(rename = "Diff", default, skip_serializing_if = "Option::is_none")]
        pub diff: Option<bool>,
        #[serde(rename = "Job", default, skip_serializing_if = "Option::is_none")]
        pub job: Option<Job>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "PolicyOverride",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub policy_override: Option<bool>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
    }

    impl From<&JobPlanRequest> for JobPlanRequest {
        fn from(value: &JobPlanRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobPlanResponse {
        #[serde(
            rename = "Annotations",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub annotations: Option<PlanAnnotations>,
        #[serde(
            rename = "CreatedEvals",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub created_evals: Vec<Evaluation>,
        #[serde(rename = "Diff", default, skip_serializing_if = "Option::is_none")]
        pub diff: Option<JobDiff>,
        #[serde(
            rename = "FailedTGAllocs",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub failed_tg_allocs: std::collections::HashMap<String, AllocationMetric>,
        #[serde(
            rename = "JobModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_modify_index: Option<u64>,
        #[serde(
            rename = "NextPeriodicLaunch",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub next_periodic_launch: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(rename = "Warnings", default, skip_serializing_if = "Option::is_none")]
        pub warnings: Option<String>,
    }

    impl From<&JobPlanResponse> for JobPlanResponse {
        fn from(value: &JobPlanResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobRegisterRequest {
        #[serde(
            rename = "EnforceIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enforce_index: Option<bool>,
        #[serde(
            rename = "EvalPriority",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eval_priority: Option<i64>,
        #[serde(rename = "Job", default, skip_serializing_if = "Option::is_none")]
        pub job: Option<Job>,
        #[serde(
            rename = "JobModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_modify_index: Option<u64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "PolicyOverride",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub policy_override: Option<bool>,
        #[serde(
            rename = "PreserveCounts",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub preserve_counts: Option<bool>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
    }

    impl From<&JobRegisterRequest> for JobRegisterRequest {
        fn from(value: &JobRegisterRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobRegisterResponse {
        #[serde(
            rename = "EvalCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eval_create_index: Option<u64>,
        #[serde(rename = "EvalID", default, skip_serializing_if = "Option::is_none")]
        pub eval_id: Option<String>,
        #[serde(
            rename = "JobModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_modify_index: Option<u64>,
        #[serde(
            rename = "KnownLeader",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub known_leader: Option<bool>,
        #[serde(
            rename = "LastContact",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_contact: Option<i64>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
        #[serde(rename = "Warnings", default, skip_serializing_if = "Option::is_none")]
        pub warnings: Option<String>,
    }

    impl From<&JobRegisterResponse> for JobRegisterResponse {
        fn from(value: &JobRegisterResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobRevertRequest {
        #[serde(
            rename = "ConsulToken",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub consul_token: Option<String>,
        #[serde(
            rename = "EnforcePriorVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enforce_prior_version: Option<u64>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(
            rename = "JobVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_version: Option<u64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
        #[serde(
            rename = "VaultToken",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub vault_token: Option<String>,
    }

    impl From<&JobRevertRequest> for JobRevertRequest {
        fn from(value: &JobRevertRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobScaleStatusResponse {
        #[serde(
            rename = "JobCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_create_index: Option<u64>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(
            rename = "JobModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_modify_index: Option<u64>,
        #[serde(
            rename = "JobStopped",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_stopped: Option<bool>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "TaskGroups",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub task_groups: std::collections::HashMap<String, TaskGroupScaleStatus>,
    }

    impl From<&JobScaleStatusResponse> for JobScaleStatusResponse {
        fn from(value: &JobScaleStatusResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobStabilityRequest {
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(
            rename = "JobVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_version: Option<u64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
        #[serde(rename = "Stable", default, skip_serializing_if = "Option::is_none")]
        pub stable: Option<bool>,
    }

    impl From<&JobStabilityRequest> for JobStabilityRequest {
        fn from(value: &JobStabilityRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobStabilityResponse {
        #[serde(rename = "Index", default, skip_serializing_if = "Option::is_none")]
        pub index: Option<u64>,
    }

    impl From<&JobStabilityResponse> for JobStabilityResponse {
        fn from(value: &JobStabilityResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobSummary {
        #[serde(rename = "Children", default, skip_serializing_if = "Option::is_none")]
        pub children: Option<JobChildrenSummary>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "Summary",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub summary: std::collections::HashMap<String, TaskGroupSummary>,
    }

    impl From<&JobSummary> for JobSummary {
        fn from(value: &JobSummary) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobValidateRequest {
        #[serde(rename = "Job", default, skip_serializing_if = "Option::is_none")]
        pub job: Option<Job>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
    }

    impl From<&JobValidateRequest> for JobValidateRequest {
        fn from(value: &JobValidateRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobValidateResponse {
        #[serde(
            rename = "DriverConfigValidated",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub driver_config_validated: Option<bool>,
        #[serde(rename = "Error", default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        #[serde(
            rename = "ValidationErrors",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub validation_errors: Vec<String>,
        #[serde(rename = "Warnings", default, skip_serializing_if = "Option::is_none")]
        pub warnings: Option<String>,
    }

    impl From<&JobValidateResponse> for JobValidateResponse {
        fn from(value: &JobValidateResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobVersionsResponse {
        #[serde(rename = "Diffs", default, skip_serializing_if = "Vec::is_empty")]
        pub diffs: Vec<JobDiff>,
        #[serde(
            rename = "KnownLeader",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub known_leader: Option<bool>,
        #[serde(
            rename = "LastContact",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_contact: Option<i64>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
        #[serde(rename = "Versions", default, skip_serializing_if = "Vec::is_empty")]
        pub versions: Vec<Job>,
    }

    impl From<&JobVersionsResponse> for JobVersionsResponse {
        fn from(value: &JobVersionsResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JobsParseRequest {
        #[serde(
            rename = "Canonicalize",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub canonicalize: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hclv1: Option<bool>,
        #[serde(rename = "JobHCL", default, skip_serializing_if = "Option::is_none")]
        pub job_hcl: Option<String>,
    }

    impl From<&JobsParseRequest> for JobsParseRequest {
        fn from(value: &JobsParseRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LogConfig {
        #[serde(
            rename = "MaxFileSizeMB",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_file_size_mb: Option<i64>,
        #[serde(rename = "MaxFiles", default, skip_serializing_if = "Option::is_none")]
        pub max_files: Option<i64>,
    }

    impl From<&LogConfig> for LogConfig {
        fn from(value: &LogConfig) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MetricsSummary {
        #[serde(rename = "Counters", default, skip_serializing_if = "Vec::is_empty")]
        pub counters: Vec<SampledValue>,
        #[serde(rename = "Gauges", default, skip_serializing_if = "Vec::is_empty")]
        pub gauges: Vec<GaugeValue>,
        #[serde(rename = "Points", default, skip_serializing_if = "Vec::is_empty")]
        pub points: Vec<PointValue>,
        #[serde(rename = "Samples", default, skip_serializing_if = "Vec::is_empty")]
        pub samples: Vec<SampledValue>,
        #[serde(rename = "Timestamp", default, skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<String>,
    }

    impl From<&MetricsSummary> for MetricsSummary {
        fn from(value: &MetricsSummary) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MigrateStrategy {
        #[serde(
            rename = "HealthCheck",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub health_check: Option<String>,
        #[serde(
            rename = "HealthyDeadline",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub healthy_deadline: Option<i64>,
        #[serde(
            rename = "MaxParallel",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_parallel: Option<i64>,
        #[serde(
            rename = "MinHealthyTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub min_healthy_time: Option<i64>,
    }

    impl From<&MigrateStrategy> for MigrateStrategy {
        fn from(value: &MigrateStrategy) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Multiregion {
        #[serde(rename = "Regions", default, skip_serializing_if = "Vec::is_empty")]
        pub regions: Vec<MultiregionRegion>,
        #[serde(rename = "Strategy", default, skip_serializing_if = "Option::is_none")]
        pub strategy: Option<MultiregionStrategy>,
    }

    impl From<&Multiregion> for Multiregion {
        fn from(value: &Multiregion) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MultiregionRegion {
        #[serde(rename = "Count", default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i64>,
        #[serde(rename = "Datacenters", default, skip_serializing_if = "Vec::is_empty")]
        pub datacenters: Vec<String>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&MultiregionRegion> for MultiregionRegion {
        fn from(value: &MultiregionRegion) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MultiregionStrategy {
        #[serde(
            rename = "MaxParallel",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_parallel: Option<i64>,
        #[serde(rename = "OnFailure", default, skip_serializing_if = "Option::is_none")]
        pub on_failure: Option<String>,
    }

    impl From<&MultiregionStrategy> for MultiregionStrategy {
        fn from(value: &MultiregionStrategy) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Namespace {
        #[serde(
            rename = "Capabilities",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub capabilities: Option<NamespaceCapabilities>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "Description",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<String>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Quota", default, skip_serializing_if = "Option::is_none")]
        pub quota: Option<String>,
    }

    impl From<&Namespace> for Namespace {
        fn from(value: &Namespace) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NamespaceCapabilities {
        #[serde(
            rename = "DisabledTaskDrivers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub disabled_task_drivers: Vec<String>,
        #[serde(
            rename = "EnabledTaskDrivers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub enabled_task_drivers: Vec<String>,
    }

    impl From<&NamespaceCapabilities> for NamespaceCapabilities {
        fn from(value: &NamespaceCapabilities) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NetworkResource {
        #[serde(rename = "CIDR", default, skip_serializing_if = "Option::is_none")]
        pub cidr: Option<String>,
        #[serde(rename = "Device", default, skip_serializing_if = "Option::is_none")]
        pub device: Option<String>,
        #[serde(rename = "DNS", default, skip_serializing_if = "Option::is_none")]
        pub dns: Option<DnsConfig>,
        #[serde(
            rename = "DynamicPorts",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub dynamic_ports: Vec<Port>,
        #[serde(rename = "Hostname", default, skip_serializing_if = "Option::is_none")]
        pub hostname: Option<String>,
        #[serde(rename = "IP", default, skip_serializing_if = "Option::is_none")]
        pub ip: Option<String>,
        #[serde(rename = "MBits", default, skip_serializing_if = "Option::is_none")]
        pub m_bits: Option<i64>,
        #[serde(rename = "Mode", default, skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,
        #[serde(
            rename = "ReservedPorts",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub reserved_ports: Vec<Port>,
    }

    impl From<&NetworkResource> for NetworkResource {
        fn from(value: &NetworkResource) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Node {
        #[serde(
            rename = "Attributes",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub attributes: std::collections::HashMap<String, String>,
        #[serde(
            rename = "CgroupParent",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub cgroup_parent: Option<String>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "CSIControllerPlugins",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub csi_controller_plugins: std::collections::HashMap<String, CsiInfo>,
        #[serde(
            rename = "CSINodePlugins",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub csi_node_plugins: std::collections::HashMap<String, CsiInfo>,
        #[serde(
            rename = "Datacenter",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub datacenter: Option<String>,
        #[serde(rename = "Drain", default, skip_serializing_if = "Option::is_none")]
        pub drain: Option<bool>,
        #[serde(
            rename = "DrainStrategy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub drain_strategy: Option<DrainStrategy>,
        #[serde(
            rename = "Drivers",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub drivers: std::collections::HashMap<String, DriverInfo>,
        #[serde(rename = "Events", default, skip_serializing_if = "Vec::is_empty")]
        pub events: Vec<NodeEvent>,
        #[serde(
            rename = "HostNetworks",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub host_networks: std::collections::HashMap<String, HostNetworkInfo>,
        #[serde(
            rename = "HostVolumes",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub host_volumes: std::collections::HashMap<String, HostVolumeInfo>,
        #[serde(rename = "HTTPAddr", default, skip_serializing_if = "Option::is_none")]
        pub http_addr: Option<String>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "LastDrain", default, skip_serializing_if = "Option::is_none")]
        pub last_drain: Option<DrainMetadata>,
        #[serde(
            rename = "Links",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub links: std::collections::HashMap<String, String>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "NodeClass", default, skip_serializing_if = "Option::is_none")]
        pub node_class: Option<String>,
        #[serde(
            rename = "NodeResources",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub node_resources: Option<NodeResources>,
        #[serde(rename = "Reserved", default, skip_serializing_if = "Option::is_none")]
        pub reserved: Option<Resources>,
        #[serde(
            rename = "ReservedResources",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reserved_resources: Option<NodeReservedResources>,
        #[serde(rename = "Resources", default, skip_serializing_if = "Option::is_none")]
        pub resources: Option<Resources>,
        #[serde(
            rename = "SchedulingEligibility",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scheduling_eligibility: Option<String>,
        #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(
            rename = "StatusDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub status_description: Option<String>,
        #[serde(
            rename = "StatusUpdatedAt",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub status_updated_at: Option<i64>,
        #[serde(
            rename = "TLSEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tls_enabled: Option<bool>,
    }

    impl From<&Node> for Node {
        fn from(value: &Node) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeCpuResources {
        #[serde(rename = "CpuShares", default, skip_serializing_if = "Option::is_none")]
        pub cpu_shares: Option<i64>,
        #[serde(
            rename = "ReservableCpuCores",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub reservable_cpu_cores: Vec<u16>,
        #[serde(
            rename = "TotalCpuCores",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub total_cpu_cores: Option<u16>,
    }

    impl From<&NodeCpuResources> for NodeCpuResources {
        fn from(value: &NodeCpuResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeDevice {
        #[serde(
            rename = "HealthDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub health_description: Option<String>,
        #[serde(rename = "Healthy", default, skip_serializing_if = "Option::is_none")]
        pub healthy: Option<bool>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "Locality", default, skip_serializing_if = "Option::is_none")]
        pub locality: Option<NodeDeviceLocality>,
    }

    impl From<&NodeDevice> for NodeDevice {
        fn from(value: &NodeDevice) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeDeviceLocality {
        #[serde(rename = "PciBusID", default, skip_serializing_if = "Option::is_none")]
        pub pci_bus_id: Option<String>,
    }

    impl From<&NodeDeviceLocality> for NodeDeviceLocality {
        fn from(value: &NodeDeviceLocality) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeDeviceResource {
        #[serde(
            rename = "Attributes",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub attributes: std::collections::HashMap<String, Attribute>,
        #[serde(rename = "Instances", default, skip_serializing_if = "Vec::is_empty")]
        pub instances: Vec<NodeDevice>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(rename = "Vendor", default, skip_serializing_if = "Option::is_none")]
        pub vendor: Option<String>,
    }

    impl From<&NodeDeviceResource> for NodeDeviceResource {
        fn from(value: &NodeDeviceResource) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeDiskResources {
        #[serde(rename = "DiskMB", default, skip_serializing_if = "Option::is_none")]
        pub disk_mb: Option<i64>,
    }

    impl From<&NodeDiskResources> for NodeDiskResources {
        fn from(value: &NodeDiskResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeDrainUpdateResponse {
        #[serde(
            rename = "EvalCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eval_create_index: Option<u64>,
        #[serde(rename = "EvalIDs", default, skip_serializing_if = "Vec::is_empty")]
        pub eval_i_ds: Vec<String>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(
            rename = "NodeModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub node_modify_index: Option<u64>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
    }

    impl From<&NodeDrainUpdateResponse> for NodeDrainUpdateResponse {
        fn from(value: &NodeDrainUpdateResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeEligibilityUpdateResponse {
        #[serde(
            rename = "EvalCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eval_create_index: Option<u64>,
        #[serde(rename = "EvalIDs", default, skip_serializing_if = "Vec::is_empty")]
        pub eval_i_ds: Vec<String>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(
            rename = "NodeModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub node_modify_index: Option<u64>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
    }

    impl From<&NodeEligibilityUpdateResponse> for NodeEligibilityUpdateResponse {
        fn from(value: &NodeEligibilityUpdateResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeEvent {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "Details",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub details: std::collections::HashMap<String, String>,
        #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(rename = "Subsystem", default, skip_serializing_if = "Option::is_none")]
        pub subsystem: Option<String>,
        #[serde(rename = "Timestamp", default, skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    }

    impl From<&NodeEvent> for NodeEvent {
        fn from(value: &NodeEvent) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeListStub {
        #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(
            rename = "Attributes",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub attributes: std::collections::HashMap<String, String>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "Datacenter",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub datacenter: Option<String>,
        #[serde(rename = "Drain", default, skip_serializing_if = "Option::is_none")]
        pub drain: Option<bool>,
        #[serde(
            rename = "Drivers",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub drivers: std::collections::HashMap<String, DriverInfo>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "LastDrain", default, skip_serializing_if = "Option::is_none")]
        pub last_drain: Option<DrainMetadata>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "NodeClass", default, skip_serializing_if = "Option::is_none")]
        pub node_class: Option<String>,
        #[serde(
            rename = "NodeResources",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub node_resources: Option<NodeResources>,
        #[serde(
            rename = "ReservedResources",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reserved_resources: Option<NodeReservedResources>,
        #[serde(
            rename = "SchedulingEligibility",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scheduling_eligibility: Option<String>,
        #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(
            rename = "StatusDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub status_description: Option<String>,
        #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }

    impl From<&NodeListStub> for NodeListStub {
        fn from(value: &NodeListStub) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeMemoryResources {
        #[serde(rename = "MemoryMB", default, skip_serializing_if = "Option::is_none")]
        pub memory_mb: Option<i64>,
    }

    impl From<&NodeMemoryResources> for NodeMemoryResources {
        fn from(value: &NodeMemoryResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodePurgeResponse {
        #[serde(
            rename = "EvalCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eval_create_index: Option<u64>,
        #[serde(rename = "EvalIDs", default, skip_serializing_if = "Vec::is_empty")]
        pub eval_i_ds: Vec<String>,
        #[serde(
            rename = "NodeModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub node_modify_index: Option<u64>,
    }

    impl From<&NodePurgeResponse> for NodePurgeResponse {
        fn from(value: &NodePurgeResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeReservedCpuResources {
        #[serde(rename = "CpuShares", default, skip_serializing_if = "Option::is_none")]
        pub cpu_shares: Option<u64>,
    }

    impl From<&NodeReservedCpuResources> for NodeReservedCpuResources {
        fn from(value: &NodeReservedCpuResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeReservedDiskResources {
        #[serde(rename = "DiskMB", default, skip_serializing_if = "Option::is_none")]
        pub disk_mb: Option<u64>,
    }

    impl From<&NodeReservedDiskResources> for NodeReservedDiskResources {
        fn from(value: &NodeReservedDiskResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeReservedMemoryResources {
        #[serde(rename = "MemoryMB", default, skip_serializing_if = "Option::is_none")]
        pub memory_mb: Option<u64>,
    }

    impl From<&NodeReservedMemoryResources> for NodeReservedMemoryResources {
        fn from(value: &NodeReservedMemoryResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeReservedNetworkResources {
        #[serde(
            rename = "ReservedHostPorts",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reserved_host_ports: Option<String>,
    }

    impl From<&NodeReservedNetworkResources> for NodeReservedNetworkResources {
        fn from(value: &NodeReservedNetworkResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeReservedResources {
        #[serde(rename = "Cpu", default, skip_serializing_if = "Option::is_none")]
        pub cpu: Option<NodeReservedCpuResources>,
        #[serde(rename = "Disk", default, skip_serializing_if = "Option::is_none")]
        pub disk: Option<NodeReservedDiskResources>,
        #[serde(rename = "Memory", default, skip_serializing_if = "Option::is_none")]
        pub memory: Option<NodeReservedMemoryResources>,
        #[serde(rename = "Networks", default, skip_serializing_if = "Option::is_none")]
        pub networks: Option<NodeReservedNetworkResources>,
    }

    impl From<&NodeReservedResources> for NodeReservedResources {
        fn from(value: &NodeReservedResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeResources {
        #[serde(rename = "Cpu", default, skip_serializing_if = "Option::is_none")]
        pub cpu: Option<NodeCpuResources>,
        #[serde(rename = "Devices", default, skip_serializing_if = "Vec::is_empty")]
        pub devices: Vec<NodeDeviceResource>,
        #[serde(rename = "Disk", default, skip_serializing_if = "Option::is_none")]
        pub disk: Option<NodeDiskResources>,
        #[serde(
            rename = "MaxDynamicPort",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_dynamic_port: Option<i64>,
        #[serde(rename = "Memory", default, skip_serializing_if = "Option::is_none")]
        pub memory: Option<NodeMemoryResources>,
        #[serde(
            rename = "MinDynamicPort",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub min_dynamic_port: Option<i64>,
        #[serde(rename = "Networks", default, skip_serializing_if = "Vec::is_empty")]
        pub networks: Vec<NetworkResource>,
    }

    impl From<&NodeResources> for NodeResources {
        fn from(value: &NodeResources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeScoreMeta {
        #[serde(rename = "NodeID", default, skip_serializing_if = "Option::is_none")]
        pub node_id: Option<String>,
        #[serde(rename = "NormScore", default, skip_serializing_if = "Option::is_none")]
        pub norm_score: Option<f64>,
        #[serde(
            rename = "Scores",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub scores: std::collections::HashMap<String, f64>,
    }

    impl From<&NodeScoreMeta> for NodeScoreMeta {
        fn from(value: &NodeScoreMeta) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeUpdateDrainRequest {
        #[serde(rename = "DrainSpec", default, skip_serializing_if = "Option::is_none")]
        pub drain_spec: Option<DrainSpec>,
        #[serde(
            rename = "MarkEligible",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mark_eligible: Option<bool>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(rename = "NodeID", default, skip_serializing_if = "Option::is_none")]
        pub node_id: Option<String>,
    }

    impl From<&NodeUpdateDrainRequest> for NodeUpdateDrainRequest {
        fn from(value: &NodeUpdateDrainRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NodeUpdateEligibilityRequest {
        #[serde(
            rename = "Eligibility",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eligibility: Option<String>,
        #[serde(rename = "NodeID", default, skip_serializing_if = "Option::is_none")]
        pub node_id: Option<String>,
    }

    impl From<&NodeUpdateEligibilityRequest> for NodeUpdateEligibilityRequest {
        fn from(value: &NodeUpdateEligibilityRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ObjectDiff {
        #[serde(rename = "Fields", default, skip_serializing_if = "Vec::is_empty")]
        pub fields: Vec<FieldDiff>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Objects", default, skip_serializing_if = "Vec::is_empty")]
        pub objects: Vec<ObjectDiff>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ObjectDiff> for ObjectDiff {
        fn from(value: &ObjectDiff) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct OneTimeToken {
        #[serde(
            rename = "AccessorID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub accessor_id: Option<String>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(rename = "ExpiresAt", default, skip_serializing_if = "Option::is_none")]
        pub expires_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "OneTimeSecretID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub one_time_secret_id: Option<String>,
    }

    impl From<&OneTimeToken> for OneTimeToken {
        fn from(value: &OneTimeToken) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct OneTimeTokenExchangeRequest {
        #[serde(
            rename = "OneTimeSecretID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub one_time_secret_id: Option<String>,
    }

    impl From<&OneTimeTokenExchangeRequest> for OneTimeTokenExchangeRequest {
        fn from(value: &OneTimeTokenExchangeRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct OperatorHealthReply {
        #[serde(
            rename = "FailureTolerance",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub failure_tolerance: Option<i64>,
        #[serde(rename = "Healthy", default, skip_serializing_if = "Option::is_none")]
        pub healthy: Option<bool>,
        #[serde(rename = "Servers", default, skip_serializing_if = "Vec::is_empty")]
        pub servers: Vec<ServerHealth>,
    }

    impl From<&OperatorHealthReply> for OperatorHealthReply {
        fn from(value: &OperatorHealthReply) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ParameterizedJobConfig {
        #[serde(
            rename = "MetaOptional",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub meta_optional: Vec<String>,
        #[serde(
            rename = "MetaRequired",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub meta_required: Vec<String>,
        #[serde(rename = "Payload", default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<String>,
    }

    impl From<&ParameterizedJobConfig> for ParameterizedJobConfig {
        fn from(value: &ParameterizedJobConfig) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PeriodicConfig {
        #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(
            rename = "ProhibitOverlap",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub prohibit_overlap: Option<bool>,
        #[serde(rename = "Spec", default, skip_serializing_if = "Option::is_none")]
        pub spec: Option<String>,
        #[serde(rename = "SpecType", default, skip_serializing_if = "Option::is_none")]
        pub spec_type: Option<String>,
        #[serde(rename = "TimeZone", default, skip_serializing_if = "Option::is_none")]
        pub time_zone: Option<String>,
    }

    impl From<&PeriodicConfig> for PeriodicConfig {
        fn from(value: &PeriodicConfig) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PeriodicForceResponse {
        #[serde(
            rename = "EvalCreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub eval_create_index: Option<u64>,
        #[serde(rename = "EvalID", default, skip_serializing_if = "Option::is_none")]
        pub eval_id: Option<String>,
        #[serde(rename = "Index", default, skip_serializing_if = "Option::is_none")]
        pub index: Option<u64>,
    }

    impl From<&PeriodicForceResponse> for PeriodicForceResponse {
        fn from(value: &PeriodicForceResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PlanAnnotations {
        #[serde(
            rename = "DesiredTGUpdates",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub desired_tg_updates: std::collections::HashMap<String, DesiredUpdates>,
        #[serde(
            rename = "PreemptedAllocs",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub preempted_allocs: Vec<AllocationListStub>,
    }

    impl From<&PlanAnnotations> for PlanAnnotations {
        fn from(value: &PlanAnnotations) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PointValue {
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Points", default, skip_serializing_if = "Vec::is_empty")]
        pub points: Vec<f64>,
    }

    impl From<&PointValue> for PointValue {
        fn from(value: &PointValue) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Port {
        #[serde(
            rename = "HostNetwork",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub host_network: Option<String>,
        #[serde(rename = "Label", default, skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(rename = "To", default, skip_serializing_if = "Option::is_none")]
        pub to: Option<i64>,
        #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
        pub value: Option<i64>,
    }

    impl From<&Port> for Port {
        fn from(value: &Port) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PortMapping {
        #[serde(rename = "HostIP", default, skip_serializing_if = "Option::is_none")]
        pub host_ip: Option<String>,
        #[serde(rename = "Label", default, skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(rename = "To", default, skip_serializing_if = "Option::is_none")]
        pub to: Option<i64>,
        #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
        pub value: Option<i64>,
    }

    impl From<&PortMapping> for PortMapping {
        fn from(value: &PortMapping) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PreemptionConfig {
        #[serde(
            rename = "BatchSchedulerEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub batch_scheduler_enabled: Option<bool>,
        #[serde(
            rename = "ServiceSchedulerEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_scheduler_enabled: Option<bool>,
        #[serde(
            rename = "SysBatchSchedulerEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sys_batch_scheduler_enabled: Option<bool>,
        #[serde(
            rename = "SystemSchedulerEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub system_scheduler_enabled: Option<bool>,
    }

    impl From<&PreemptionConfig> for PreemptionConfig {
        fn from(value: &PreemptionConfig) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QuotaLimit {
        #[serde(rename = "Hash", default, skip_serializing_if = "Option::is_none")]
        pub hash: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(
            rename = "RegionLimit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub region_limit: Option<Resources>,
        #[serde(
            rename = "VariablesLimit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub variables_limit: Option<i64>,
    }

    impl From<&QuotaLimit> for QuotaLimit {
        fn from(value: &QuotaLimit) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct QuotaSpec {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "Description",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub description: Option<String>,
        #[serde(rename = "Limits", default, skip_serializing_if = "Vec::is_empty")]
        pub limits: Vec<QuotaLimit>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&QuotaSpec> for QuotaSpec {
        fn from(value: &QuotaSpec) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Quotas(pub serde_json::Value);
    impl std::ops::Deref for Quotas {
        type Target = serde_json::Value;
        fn deref(&self) -> &serde_json::Value {
            &self.0
        }
    }

    impl From<Quotas> for serde_json::Value {
        fn from(value: Quotas) -> Self {
            value.0
        }
    }

    impl From<&Quotas> for Quotas {
        fn from(value: &Quotas) -> Self {
            value.clone()
        }
    }

    impl From<serde_json::Value> for Quotas {
        fn from(value: serde_json::Value) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RaftConfiguration {
        #[serde(rename = "Index", default, skip_serializing_if = "Option::is_none")]
        pub index: Option<u64>,
        #[serde(rename = "Servers", default, skip_serializing_if = "Vec::is_empty")]
        pub servers: Vec<RaftServer>,
    }

    impl From<&RaftConfiguration> for RaftConfiguration {
        fn from(value: &RaftConfiguration) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RaftServer {
        #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "Leader", default, skip_serializing_if = "Option::is_none")]
        pub leader: Option<bool>,
        #[serde(rename = "Node", default, skip_serializing_if = "Option::is_none")]
        pub node: Option<String>,
        #[serde(
            rename = "RaftProtocol",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub raft_protocol: Option<String>,
        #[serde(rename = "Voter", default, skip_serializing_if = "Option::is_none")]
        pub voter: Option<bool>,
    }

    impl From<&RaftServer> for RaftServer {
        fn from(value: &RaftServer) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RequestedDevice {
        #[serde(rename = "Affinities", default, skip_serializing_if = "Vec::is_empty")]
        pub affinities: Vec<Affinity>,
        #[serde(rename = "Constraints", default, skip_serializing_if = "Vec::is_empty")]
        pub constraints: Vec<Constraint>,
        #[serde(rename = "Count", default, skip_serializing_if = "Option::is_none")]
        pub count: Option<u64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&RequestedDevice> for RequestedDevice {
        fn from(value: &RequestedDevice) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RescheduleEvent {
        #[serde(
            rename = "PrevAllocID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub prev_alloc_id: Option<String>,
        #[serde(
            rename = "PrevNodeID",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub prev_node_id: Option<String>,
        #[serde(
            rename = "RescheduleTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reschedule_time: Option<i64>,
    }

    impl From<&RescheduleEvent> for RescheduleEvent {
        fn from(value: &RescheduleEvent) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ReschedulePolicy {
        #[serde(rename = "Attempts", default, skip_serializing_if = "Option::is_none")]
        pub attempts: Option<i64>,
        #[serde(rename = "Delay", default, skip_serializing_if = "Option::is_none")]
        pub delay: Option<i64>,
        #[serde(
            rename = "DelayFunction",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub delay_function: Option<String>,
        #[serde(rename = "Interval", default, skip_serializing_if = "Option::is_none")]
        pub interval: Option<i64>,
        #[serde(rename = "MaxDelay", default, skip_serializing_if = "Option::is_none")]
        pub max_delay: Option<i64>,
        #[serde(rename = "Unlimited", default, skip_serializing_if = "Option::is_none")]
        pub unlimited: Option<bool>,
    }

    impl From<&ReschedulePolicy> for ReschedulePolicy {
        fn from(value: &ReschedulePolicy) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RescheduleTracker {
        #[serde(rename = "Events", default, skip_serializing_if = "Vec::is_empty")]
        pub events: Vec<RescheduleEvent>,
    }

    impl From<&RescheduleTracker> for RescheduleTracker {
        fn from(value: &RescheduleTracker) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Resources {
        #[serde(rename = "Cores", default, skip_serializing_if = "Option::is_none")]
        pub cores: Option<i64>,
        #[serde(rename = "CPU", default, skip_serializing_if = "Option::is_none")]
        pub cpu: Option<i64>,
        #[serde(rename = "Devices", default, skip_serializing_if = "Vec::is_empty")]
        pub devices: Vec<RequestedDevice>,
        #[serde(rename = "DiskMB", default, skip_serializing_if = "Option::is_none")]
        pub disk_mb: Option<i64>,
        #[serde(rename = "IOPS", default, skip_serializing_if = "Option::is_none")]
        pub iops: Option<i64>,
        #[serde(
            rename = "MemoryMaxMB",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub memory_max_mb: Option<i64>,
        #[serde(rename = "MemoryMB", default, skip_serializing_if = "Option::is_none")]
        pub memory_mb: Option<i64>,
        #[serde(rename = "Networks", default, skip_serializing_if = "Vec::is_empty")]
        pub networks: Vec<NetworkResource>,
    }

    impl From<&Resources> for Resources {
        fn from(value: &Resources) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RestartPolicy {
        #[serde(rename = "Attempts", default, skip_serializing_if = "Option::is_none")]
        pub attempts: Option<i64>,
        #[serde(rename = "Delay", default, skip_serializing_if = "Option::is_none")]
        pub delay: Option<i64>,
        #[serde(rename = "Interval", default, skip_serializing_if = "Option::is_none")]
        pub interval: Option<i64>,
        #[serde(rename = "Mode", default, skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,
    }

    impl From<&RestartPolicy> for RestartPolicy {
        fn from(value: &RestartPolicy) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SampledValue {
        #[serde(rename = "Count", default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i64>,
        #[serde(
            rename = "Labels",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub labels: std::collections::HashMap<String, String>,
        #[serde(rename = "Max", default, skip_serializing_if = "Option::is_none")]
        pub max: Option<f64>,
        #[serde(rename = "Mean", default, skip_serializing_if = "Option::is_none")]
        pub mean: Option<f64>,
        #[serde(rename = "Min", default, skip_serializing_if = "Option::is_none")]
        pub min: Option<f64>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Rate", default, skip_serializing_if = "Option::is_none")]
        pub rate: Option<f64>,
        #[serde(rename = "Stddev", default, skip_serializing_if = "Option::is_none")]
        pub stddev: Option<f64>,
        #[serde(rename = "Sum", default, skip_serializing_if = "Option::is_none")]
        pub sum: Option<f64>,
    }

    impl From<&SampledValue> for SampledValue {
        fn from(value: &SampledValue) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ScalingEvent {
        #[serde(rename = "Count", default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i64>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(rename = "Error", default, skip_serializing_if = "Option::is_none")]
        pub error: Option<bool>,
        #[serde(rename = "EvalID", default, skip_serializing_if = "Option::is_none")]
        pub eval_id: Option<String>,
        #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, serde_json::Value>,
        #[serde(
            rename = "PreviousCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub previous_count: Option<i64>,
        #[serde(rename = "Time", default, skip_serializing_if = "Option::is_none")]
        pub time: Option<u64>,
    }

    impl From<&ScalingEvent> for ScalingEvent {
        fn from(value: &ScalingEvent) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ScalingPolicy {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "Max", default, skip_serializing_if = "Option::is_none")]
        pub max: Option<i64>,
        #[serde(rename = "Min", default, skip_serializing_if = "Option::is_none")]
        pub min: Option<i64>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "Policy",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub policy: std::collections::HashMap<String, serde_json::Value>,
        #[serde(
            rename = "Target",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub target: std::collections::HashMap<String, String>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ScalingPolicy> for ScalingPolicy {
        fn from(value: &ScalingPolicy) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ScalingPolicyListStub {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "Target",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub target: std::collections::HashMap<String, String>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ScalingPolicyListStub> for ScalingPolicyListStub {
        fn from(value: &ScalingPolicyListStub) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ScalingRequest {
        #[serde(rename = "Count", default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i64>,
        #[serde(rename = "Error", default, skip_serializing_if = "Option::is_none")]
        pub error: Option<bool>,
        #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, serde_json::Value>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(
            rename = "PolicyOverride",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub policy_override: Option<bool>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "SecretID", default, skip_serializing_if = "Option::is_none")]
        pub secret_id: Option<String>,
        #[serde(
            rename = "Target",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub target: std::collections::HashMap<String, String>,
    }

    impl From<&ScalingRequest> for ScalingRequest {
        fn from(value: &ScalingRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct SchedulerAlgorithm(pub String);
    impl std::ops::Deref for SchedulerAlgorithm {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<SchedulerAlgorithm> for String {
        fn from(value: SchedulerAlgorithm) -> Self {
            value.0
        }
    }

    impl From<&SchedulerAlgorithm> for SchedulerAlgorithm {
        fn from(value: &SchedulerAlgorithm) -> Self {
            value.clone()
        }
    }

    impl From<String> for SchedulerAlgorithm {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for SchedulerAlgorithm {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ToString for SchedulerAlgorithm {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SchedulerConfiguration {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "MemoryOversubscriptionEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub memory_oversubscription_enabled: Option<bool>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "PauseEvalBroker",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub pause_eval_broker: Option<bool>,
        #[serde(
            rename = "PreemptionConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub preemption_config: Option<PreemptionConfig>,
        #[serde(
            rename = "RejectJobRegistration",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reject_job_registration: Option<bool>,
        #[serde(
            rename = "SchedulerAlgorithm",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scheduler_algorithm: Option<String>,
    }

    impl From<&SchedulerConfiguration> for SchedulerConfiguration {
        fn from(value: &SchedulerConfiguration) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SchedulerConfigurationResponse {
        #[serde(
            rename = "KnownLeader",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub known_leader: Option<bool>,
        #[serde(
            rename = "LastContact",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_contact: Option<i64>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
        #[serde(
            rename = "SchedulerConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scheduler_config: Option<SchedulerConfiguration>,
    }

    impl From<&SchedulerConfigurationResponse> for SchedulerConfigurationResponse {
        fn from(value: &SchedulerConfigurationResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SchedulerSetConfigurationResponse {
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
        #[serde(rename = "Updated", default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<bool>,
    }

    impl From<&SchedulerSetConfigurationResponse> for SchedulerSetConfigurationResponse {
        fn from(value: &SchedulerSetConfigurationResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SearchRequest {
        #[serde(
            rename = "AllowStale",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_stale: Option<bool>,
        #[serde(rename = "AuthToken", default, skip_serializing_if = "Option::is_none")]
        pub auth_token: Option<String>,
        #[serde(rename = "Context", default, skip_serializing_if = "Option::is_none")]
        pub context: Option<String>,
        #[serde(rename = "Filter", default, skip_serializing_if = "Option::is_none")]
        pub filter: Option<String>,
        #[serde(
            rename = "Headers",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub headers: std::collections::HashMap<String, String>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(
            rename = "Params",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub params: std::collections::HashMap<String, String>,
        #[serde(rename = "PerPage", default, skip_serializing_if = "Option::is_none")]
        pub per_page: Option<i32>,
        #[serde(rename = "Prefix", default, skip_serializing_if = "Option::is_none")]
        pub prefix: Option<String>,
        #[serde(rename = "Region", default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "Reverse", default, skip_serializing_if = "Option::is_none")]
        pub reverse: Option<bool>,
        #[serde(rename = "WaitIndex", default, skip_serializing_if = "Option::is_none")]
        pub wait_index: Option<u64>,
        #[serde(rename = "WaitTime", default, skip_serializing_if = "Option::is_none")]
        pub wait_time: Option<i64>,
    }

    impl From<&SearchRequest> for SearchRequest {
        fn from(value: &SearchRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SearchResponse {
        #[serde(
            rename = "KnownLeader",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub known_leader: Option<bool>,
        #[serde(
            rename = "LastContact",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_contact: Option<i64>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(
            rename = "Matches",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub matches: std::collections::HashMap<String, Vec<String>>,
        #[serde(rename = "NextToken", default, skip_serializing_if = "Option::is_none")]
        pub next_token: Option<String>,
        #[serde(
            rename = "RequestTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub request_time: Option<i64>,
        #[serde(
            rename = "Truncations",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub truncations: std::collections::HashMap<String, bool>,
    }

    impl From<&SearchResponse> for SearchResponse {
        fn from(value: &SearchResponse) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ServerHealth {
        #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(rename = "Healthy", default, skip_serializing_if = "Option::is_none")]
        pub healthy: Option<bool>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "LastContact",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_contact: Option<i64>,
        #[serde(rename = "LastIndex", default, skip_serializing_if = "Option::is_none")]
        pub last_index: Option<u64>,
        #[serde(rename = "LastTerm", default, skip_serializing_if = "Option::is_none")]
        pub last_term: Option<u64>,
        #[serde(rename = "Leader", default, skip_serializing_if = "Option::is_none")]
        pub leader: Option<bool>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "SerfStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub serf_status: Option<String>,
        #[serde(
            rename = "StableSince",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub stable_since: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(rename = "Voter", default, skip_serializing_if = "Option::is_none")]
        pub voter: Option<bool>,
    }

    impl From<&ServerHealth> for ServerHealth {
        fn from(value: &ServerHealth) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Service {
        #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(
            rename = "AddressMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_mode: Option<String>,
        #[serde(
            rename = "CanaryMeta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub canary_meta: std::collections::HashMap<String, String>,
        #[serde(rename = "CanaryTags", default, skip_serializing_if = "Vec::is_empty")]
        pub canary_tags: Vec<String>,
        #[serde(
            rename = "CheckRestart",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub check_restart: Option<CheckRestart>,
        #[serde(rename = "Checks", default, skip_serializing_if = "Vec::is_empty")]
        pub checks: Vec<ServiceCheck>,
        #[serde(rename = "Connect", default, skip_serializing_if = "Option::is_none")]
        pub connect: Option<ConsulConnect>,
        #[serde(
            rename = "EnableTagOverride",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_tag_override: Option<bool>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "OnUpdate", default, skip_serializing_if = "Option::is_none")]
        pub on_update: Option<String>,
        #[serde(rename = "PortLabel", default, skip_serializing_if = "Option::is_none")]
        pub port_label: Option<String>,
        #[serde(rename = "Provider", default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(
            rename = "TaggedAddresses",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub tagged_addresses: std::collections::HashMap<String, String>,
        #[serde(rename = "Tags", default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<String>,
        #[serde(rename = "TaskName", default, skip_serializing_if = "Option::is_none")]
        pub task_name: Option<String>,
    }

    impl From<&Service> for Service {
        fn from(value: &Service) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ServiceCheck {
        #[serde(
            rename = "AddressMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub address_mode: Option<String>,
        #[serde(rename = "Advertise", default, skip_serializing_if = "Option::is_none")]
        pub advertise: Option<String>,
        #[serde(rename = "Args", default, skip_serializing_if = "Vec::is_empty")]
        pub args: Vec<String>,
        #[serde(rename = "Body", default, skip_serializing_if = "Option::is_none")]
        pub body: Option<String>,
        #[serde(
            rename = "CheckRestart",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub check_restart: Option<CheckRestart>,
        #[serde(rename = "Command", default, skip_serializing_if = "Option::is_none")]
        pub command: Option<String>,
        #[serde(rename = "Expose", default, skip_serializing_if = "Option::is_none")]
        pub expose: Option<bool>,
        #[serde(
            rename = "FailuresBeforeCritical",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub failures_before_critical: Option<i64>,
        #[serde(
            rename = "GRPCService",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub grpc_service: Option<String>,
        #[serde(
            rename = "GRPCUseTLS",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub grpc_use_tls: Option<bool>,
        #[serde(
            rename = "Header",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub header: std::collections::HashMap<String, Vec<String>>,
        #[serde(
            rename = "InitialStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub initial_status: Option<String>,
        #[serde(rename = "Interval", default, skip_serializing_if = "Option::is_none")]
        pub interval: Option<i64>,
        #[serde(rename = "Method", default, skip_serializing_if = "Option::is_none")]
        pub method: Option<String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "OnUpdate", default, skip_serializing_if = "Option::is_none")]
        pub on_update: Option<String>,
        #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(rename = "PortLabel", default, skip_serializing_if = "Option::is_none")]
        pub port_label: Option<String>,
        #[serde(rename = "Protocol", default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
        #[serde(
            rename = "SuccessBeforePassing",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub success_before_passing: Option<i64>,
        #[serde(rename = "TaskName", default, skip_serializing_if = "Option::is_none")]
        pub task_name: Option<String>,
        #[serde(rename = "Timeout", default, skip_serializing_if = "Option::is_none")]
        pub timeout: Option<i64>,
        #[serde(
            rename = "TLSSkipVerify",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tls_skip_verify: Option<bool>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ServiceCheck> for ServiceCheck {
        fn from(value: &ServiceCheck) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ServiceRegistration {
        #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(rename = "AllocID", default, skip_serializing_if = "Option::is_none")]
        pub alloc_id: Option<String>,
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "Datacenter",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub datacenter: Option<String>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "JobID", default, skip_serializing_if = "Option::is_none")]
        pub job_id: Option<String>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "NodeID", default, skip_serializing_if = "Option::is_none")]
        pub node_id: Option<String>,
        #[serde(rename = "Port", default, skip_serializing_if = "Option::is_none")]
        pub port: Option<i64>,
        #[serde(
            rename = "ServiceName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_name: Option<String>,
        #[serde(rename = "Tags", default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<String>,
    }

    impl From<&ServiceRegistration> for ServiceRegistration {
        fn from(value: &ServiceRegistration) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SidecarTask {
        #[serde(
            rename = "Config",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub config: std::collections::HashMap<String, serde_json::Value>,
        #[serde(rename = "Driver", default, skip_serializing_if = "Option::is_none")]
        pub driver: Option<String>,
        #[serde(
            rename = "Env",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub env: std::collections::HashMap<String, String>,
        #[serde(
            rename = "KillSignal",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub kill_signal: Option<String>,
        #[serde(
            rename = "KillTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub kill_timeout: Option<i64>,
        #[serde(rename = "LogConfig", default, skip_serializing_if = "Option::is_none")]
        pub log_config: Option<LogConfig>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Resources", default, skip_serializing_if = "Option::is_none")]
        pub resources: Option<Resources>,
        #[serde(
            rename = "ShutdownDelay",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shutdown_delay: Option<i64>,
        #[serde(rename = "User", default, skip_serializing_if = "Option::is_none")]
        pub user: Option<String>,
    }

    impl From<&SidecarTask> for SidecarTask {
        fn from(value: &SidecarTask) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Spread {
        #[serde(rename = "Attribute", default, skip_serializing_if = "Option::is_none")]
        pub attribute: Option<String>,
        #[serde(
            rename = "SpreadTarget",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub spread_target: Vec<SpreadTarget>,
        #[serde(rename = "Weight", default, skip_serializing_if = "Option::is_none")]
        pub weight: Option<i8>,
    }

    impl From<&Spread> for Spread {
        fn from(value: &Spread) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SpreadTarget {
        #[serde(rename = "Percent", default, skip_serializing_if = "Option::is_none")]
        pub percent: Option<u8>,
        #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    impl From<&SpreadTarget> for SpreadTarget {
        fn from(value: &SpreadTarget) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Task {
        #[serde(rename = "Affinities", default, skip_serializing_if = "Vec::is_empty")]
        pub affinities: Vec<Affinity>,
        #[serde(rename = "Artifacts", default, skip_serializing_if = "Vec::is_empty")]
        pub artifacts: Vec<TaskArtifact>,
        #[serde(
            rename = "Config",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub config: std::collections::HashMap<String, serde_json::Value>,
        #[serde(rename = "Constraints", default, skip_serializing_if = "Vec::is_empty")]
        pub constraints: Vec<Constraint>,
        #[serde(
            rename = "CSIPluginConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub csi_plugin_config: Option<TaskCsiPluginConfig>,
        #[serde(
            rename = "DispatchPayload",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub dispatch_payload: Option<DispatchPayloadConfig>,
        #[serde(rename = "Driver", default, skip_serializing_if = "Option::is_none")]
        pub driver: Option<String>,
        #[serde(
            rename = "Env",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub env: std::collections::HashMap<String, String>,
        #[serde(
            rename = "KillSignal",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub kill_signal: Option<String>,
        #[serde(
            rename = "KillTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub kill_timeout: Option<i64>,
        #[serde(rename = "Kind", default, skip_serializing_if = "Option::is_none")]
        pub kind: Option<String>,
        #[serde(rename = "Leader", default, skip_serializing_if = "Option::is_none")]
        pub leader: Option<bool>,
        #[serde(rename = "Lifecycle", default, skip_serializing_if = "Option::is_none")]
        pub lifecycle: Option<TaskLifecycle>,
        #[serde(rename = "LogConfig", default, skip_serializing_if = "Option::is_none")]
        pub log_config: Option<LogConfig>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Resources", default, skip_serializing_if = "Option::is_none")]
        pub resources: Option<Resources>,
        #[serde(
            rename = "RestartPolicy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub restart_policy: Option<RestartPolicy>,
        #[serde(
            rename = "ScalingPolicies",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub scaling_policies: Vec<ScalingPolicy>,
        #[serde(rename = "Services", default, skip_serializing_if = "Vec::is_empty")]
        pub services: Vec<Service>,
        #[serde(
            rename = "ShutdownDelay",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shutdown_delay: Option<i64>,
        #[serde(rename = "Templates", default, skip_serializing_if = "Vec::is_empty")]
        pub templates: Vec<Template>,
        #[serde(rename = "User", default, skip_serializing_if = "Option::is_none")]
        pub user: Option<String>,
        #[serde(rename = "Vault", default, skip_serializing_if = "Option::is_none")]
        pub vault: Option<Vault>,
        #[serde(
            rename = "VolumeMounts",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub volume_mounts: Vec<VolumeMount>,
    }

    impl From<&Task> for Task {
        fn from(value: &Task) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskArtifact {
        #[serde(
            rename = "GetterHeaders",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub getter_headers: std::collections::HashMap<String, String>,
        #[serde(
            rename = "GetterMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub getter_mode: Option<String>,
        #[serde(
            rename = "GetterOptions",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub getter_options: std::collections::HashMap<String, String>,
        #[serde(
            rename = "GetterSource",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub getter_source: Option<String>,
        #[serde(
            rename = "RelativeDest",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub relative_dest: Option<String>,
    }

    impl From<&TaskArtifact> for TaskArtifact {
        fn from(value: &TaskArtifact) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskCsiPluginConfig {
        #[serde(
            rename = "HealthTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub health_timeout: Option<i64>,
        #[serde(rename = "ID", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "MountDir", default, skip_serializing_if = "Option::is_none")]
        pub mount_dir: Option<String>,
        #[serde(
            rename = "StagePublishBaseDir",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub stage_publish_base_dir: Option<String>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&TaskCsiPluginConfig> for TaskCsiPluginConfig {
        fn from(value: &TaskCsiPluginConfig) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskDiff {
        #[serde(rename = "Annotations", default, skip_serializing_if = "Vec::is_empty")]
        pub annotations: Vec<String>,
        #[serde(rename = "Fields", default, skip_serializing_if = "Vec::is_empty")]
        pub fields: Vec<FieldDiff>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Objects", default, skip_serializing_if = "Vec::is_empty")]
        pub objects: Vec<ObjectDiff>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&TaskDiff> for TaskDiff {
        fn from(value: &TaskDiff) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskEvent {
        #[serde(
            rename = "Details",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub details: std::collections::HashMap<String, String>,
        #[serde(rename = "DiskLimit", default, skip_serializing_if = "Option::is_none")]
        pub disk_limit: Option<i64>,
        #[serde(rename = "DiskSize", default, skip_serializing_if = "Option::is_none")]
        pub disk_size: Option<i64>,
        #[serde(
            rename = "DisplayMessage",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_message: Option<String>,
        #[serde(
            rename = "DownloadError",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub download_error: Option<String>,
        #[serde(
            rename = "DriverError",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub driver_error: Option<String>,
        #[serde(
            rename = "DriverMessage",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub driver_message: Option<String>,
        #[serde(rename = "ExitCode", default, skip_serializing_if = "Option::is_none")]
        pub exit_code: Option<i64>,
        #[serde(
            rename = "FailedSibling",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub failed_sibling: Option<String>,
        #[serde(rename = "FailsTask", default, skip_serializing_if = "Option::is_none")]
        pub fails_task: Option<bool>,
        #[serde(
            rename = "GenericSource",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub generic_source: Option<String>,
        #[serde(rename = "KillError", default, skip_serializing_if = "Option::is_none")]
        pub kill_error: Option<String>,
        #[serde(
            rename = "KillReason",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub kill_reason: Option<String>,
        #[serde(
            rename = "KillTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub kill_timeout: Option<i64>,
        #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(
            rename = "RestartReason",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub restart_reason: Option<String>,
        #[serde(
            rename = "SetupError",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub setup_error: Option<String>,
        #[serde(rename = "Signal", default, skip_serializing_if = "Option::is_none")]
        pub signal: Option<i64>,
        #[serde(
            rename = "StartDelay",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub start_delay: Option<i64>,
        #[serde(
            rename = "TaskSignal",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub task_signal: Option<String>,
        #[serde(
            rename = "TaskSignalReason",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub task_signal_reason: Option<String>,
        #[serde(rename = "Time", default, skip_serializing_if = "Option::is_none")]
        pub time: Option<i64>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(
            rename = "ValidationError",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub validation_error: Option<String>,
        #[serde(
            rename = "VaultError",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub vault_error: Option<String>,
    }

    impl From<&TaskEvent> for TaskEvent {
        fn from(value: &TaskEvent) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskGroup {
        #[serde(rename = "Affinities", default, skip_serializing_if = "Vec::is_empty")]
        pub affinities: Vec<Affinity>,
        #[serde(rename = "Constraints", default, skip_serializing_if = "Vec::is_empty")]
        pub constraints: Vec<Constraint>,
        #[serde(rename = "Consul", default, skip_serializing_if = "Option::is_none")]
        pub consul: Option<Consul>,
        #[serde(rename = "Count", default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i64>,
        #[serde(
            rename = "EphemeralDisk",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ephemeral_disk: Option<EphemeralDisk>,
        #[serde(
            rename = "MaxClientDisconnect",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_client_disconnect: Option<i64>,
        #[serde(
            rename = "Meta",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub meta: std::collections::HashMap<String, String>,
        #[serde(rename = "Migrate", default, skip_serializing_if = "Option::is_none")]
        pub migrate: Option<MigrateStrategy>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Networks", default, skip_serializing_if = "Vec::is_empty")]
        pub networks: Vec<NetworkResource>,
        #[serde(
            rename = "ReschedulePolicy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reschedule_policy: Option<ReschedulePolicy>,
        #[serde(
            rename = "RestartPolicy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub restart_policy: Option<RestartPolicy>,
        #[serde(rename = "Scaling", default, skip_serializing_if = "Option::is_none")]
        pub scaling: Option<ScalingPolicy>,
        #[serde(rename = "Services", default, skip_serializing_if = "Vec::is_empty")]
        pub services: Vec<Service>,
        #[serde(
            rename = "ShutdownDelay",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub shutdown_delay: Option<i64>,
        #[serde(rename = "Spreads", default, skip_serializing_if = "Vec::is_empty")]
        pub spreads: Vec<Spread>,
        #[serde(
            rename = "StopAfterClientDisconnect",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub stop_after_client_disconnect: Option<i64>,
        #[serde(rename = "Tasks", default, skip_serializing_if = "Vec::is_empty")]
        pub tasks: Vec<Task>,
        #[serde(rename = "Update", default, skip_serializing_if = "Option::is_none")]
        pub update: Option<UpdateStrategy>,
        #[serde(
            rename = "Volumes",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub volumes: std::collections::HashMap<String, VolumeRequest>,
    }

    impl From<&TaskGroup> for TaskGroup {
        fn from(value: &TaskGroup) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskGroupDiff {
        #[serde(rename = "Fields", default, skip_serializing_if = "Vec::is_empty")]
        pub fields: Vec<FieldDiff>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Objects", default, skip_serializing_if = "Vec::is_empty")]
        pub objects: Vec<ObjectDiff>,
        #[serde(rename = "Tasks", default, skip_serializing_if = "Vec::is_empty")]
        pub tasks: Vec<TaskDiff>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(
            rename = "Updates",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub updates: std::collections::HashMap<String, u64>,
    }

    impl From<&TaskGroupDiff> for TaskGroupDiff {
        fn from(value: &TaskGroupDiff) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskGroupScaleStatus {
        #[serde(rename = "Desired", default, skip_serializing_if = "Option::is_none")]
        pub desired: Option<i64>,
        #[serde(rename = "Events", default, skip_serializing_if = "Vec::is_empty")]
        pub events: Vec<ScalingEvent>,
        #[serde(rename = "Healthy", default, skip_serializing_if = "Option::is_none")]
        pub healthy: Option<i64>,
        #[serde(rename = "Placed", default, skip_serializing_if = "Option::is_none")]
        pub placed: Option<i64>,
        #[serde(rename = "Running", default, skip_serializing_if = "Option::is_none")]
        pub running: Option<i64>,
        #[serde(rename = "Unhealthy", default, skip_serializing_if = "Option::is_none")]
        pub unhealthy: Option<i64>,
    }

    impl From<&TaskGroupScaleStatus> for TaskGroupScaleStatus {
        fn from(value: &TaskGroupScaleStatus) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskGroupSummary {
        #[serde(rename = "Complete", default, skip_serializing_if = "Option::is_none")]
        pub complete: Option<i64>,
        #[serde(rename = "Failed", default, skip_serializing_if = "Option::is_none")]
        pub failed: Option<i64>,
        #[serde(rename = "Lost", default, skip_serializing_if = "Option::is_none")]
        pub lost: Option<i64>,
        #[serde(rename = "Queued", default, skip_serializing_if = "Option::is_none")]
        pub queued: Option<i64>,
        #[serde(rename = "Running", default, skip_serializing_if = "Option::is_none")]
        pub running: Option<i64>,
        #[serde(rename = "Starting", default, skip_serializing_if = "Option::is_none")]
        pub starting: Option<i64>,
        #[serde(rename = "Unknown", default, skip_serializing_if = "Option::is_none")]
        pub unknown: Option<i64>,
    }

    impl From<&TaskGroupSummary> for TaskGroupSummary {
        fn from(value: &TaskGroupSummary) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskHandle {
        #[serde(
            rename = "DriverState",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub driver_state: Option<String>,
        #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
        pub version: Option<i64>,
    }

    impl From<&TaskHandle> for TaskHandle {
        fn from(value: &TaskHandle) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskLifecycle {
        #[serde(rename = "Hook", default, skip_serializing_if = "Option::is_none")]
        pub hook: Option<String>,
        #[serde(rename = "Sidecar", default, skip_serializing_if = "Option::is_none")]
        pub sidecar: Option<bool>,
    }

    impl From<&TaskLifecycle> for TaskLifecycle {
        fn from(value: &TaskLifecycle) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskState {
        #[serde(rename = "Events", default, skip_serializing_if = "Vec::is_empty")]
        pub events: Vec<TaskEvent>,
        #[serde(rename = "Failed", default, skip_serializing_if = "Option::is_none")]
        pub failed: Option<bool>,
        #[serde(
            rename = "FinishedAt",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub finished_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(
            rename = "LastRestart",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_restart: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(rename = "Restarts", default, skip_serializing_if = "Option::is_none")]
        pub restarts: Option<u64>,
        #[serde(rename = "StartedAt", default, skip_serializing_if = "Option::is_none")]
        pub started_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(rename = "State", default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        #[serde(
            rename = "TaskHandle",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub task_handle: Option<TaskHandle>,
    }

    impl From<&TaskState> for TaskState {
        fn from(value: &TaskState) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Template {
        #[serde(
            rename = "ChangeMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub change_mode: Option<String>,
        #[serde(
            rename = "ChangeScript",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub change_script: Option<ChangeScript>,
        #[serde(
            rename = "ChangeSignal",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub change_signal: Option<String>,
        #[serde(rename = "DestPath", default, skip_serializing_if = "Option::is_none")]
        pub dest_path: Option<String>,
        #[serde(
            rename = "EmbeddedTmpl",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub embedded_tmpl: Option<String>,
        #[serde(rename = "Envvars", default, skip_serializing_if = "Option::is_none")]
        pub envvars: Option<bool>,
        #[serde(rename = "Gid", default, skip_serializing_if = "Option::is_none")]
        pub gid: Option<i64>,
        #[serde(rename = "LeftDelim", default, skip_serializing_if = "Option::is_none")]
        pub left_delim: Option<String>,
        #[serde(rename = "Perms", default, skip_serializing_if = "Option::is_none")]
        pub perms: Option<String>,
        #[serde(
            rename = "RightDelim",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub right_delim: Option<String>,
        #[serde(
            rename = "SourcePath",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub source_path: Option<String>,
        #[serde(rename = "Splay", default, skip_serializing_if = "Option::is_none")]
        pub splay: Option<i64>,
        #[serde(rename = "Uid", default, skip_serializing_if = "Option::is_none")]
        pub uid: Option<i64>,
        #[serde(
            rename = "VaultGrace",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub vault_grace: Option<i64>,
        #[serde(rename = "Wait", default, skip_serializing_if = "Option::is_none")]
        pub wait: Option<WaitConfig>,
    }

    impl From<&Template> for Template {
        fn from(value: &Template) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Time(pub chrono::DateTime<chrono::offset::Utc>);
    impl std::ops::Deref for Time {
        type Target = chrono::DateTime<chrono::offset::Utc>;
        fn deref(&self) -> &chrono::DateTime<chrono::offset::Utc> {
            &self.0
        }
    }

    impl From<Time> for chrono::DateTime<chrono::offset::Utc> {
        fn from(value: Time) -> Self {
            value.0
        }
    }

    impl From<&Time> for Time {
        fn from(value: &Time) -> Self {
            value.clone()
        }
    }

    impl From<chrono::DateTime<chrono::offset::Utc>> for Time {
        fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Time {
        type Err = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Time {
        type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Time {
        type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Time {
        type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Time {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Uint(pub u64);
    impl std::ops::Deref for Uint {
        type Target = u64;
        fn deref(&self) -> &u64 {
            &self.0
        }
    }

    impl From<Uint> for u64 {
        fn from(value: Uint) -> Self {
            value.0
        }
    }

    impl From<&Uint> for Uint {
        fn from(value: &Uint) -> Self {
            value.clone()
        }
    }

    impl From<u64> for Uint {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Uint {
        type Err = <u64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Uint {
        type Error = <u64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Uint {
        type Error = <u64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Uint {
        type Error = <u64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Uint {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Uint16(pub u16);
    impl std::ops::Deref for Uint16 {
        type Target = u16;
        fn deref(&self) -> &u16 {
            &self.0
        }
    }

    impl From<Uint16> for u16 {
        fn from(value: Uint16) -> Self {
            value.0
        }
    }

    impl From<&Uint16> for Uint16 {
        fn from(value: &Uint16) -> Self {
            value.clone()
        }
    }

    impl From<u16> for Uint16 {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Uint16 {
        type Err = <u16 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Uint16 {
        type Error = <u16 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Uint16 {
        type Error = <u16 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Uint16 {
        type Error = <u16 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Uint16 {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Uint64(pub u64);
    impl std::ops::Deref for Uint64 {
        type Target = u64;
        fn deref(&self) -> &u64 {
            &self.0
        }
    }

    impl From<Uint64> for u64 {
        fn from(value: Uint64) -> Self {
            value.0
        }
    }

    impl From<&Uint64> for Uint64 {
        fn from(value: &Uint64) -> Self {
            value.clone()
        }
    }

    impl From<u64> for Uint64 {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Uint64 {
        type Err = <u64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Uint64 {
        type Error = <u64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Uint64 {
        type Error = <u64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Uint64 {
        type Error = <u64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Uint64 {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Uint8(pub u8);
    impl std::ops::Deref for Uint8 {
        type Target = u8;
        fn deref(&self) -> &u8 {
            &self.0
        }
    }

    impl From<Uint8> for u8 {
        fn from(value: Uint8) -> Self {
            value.0
        }
    }

    impl From<&Uint8> for Uint8 {
        fn from(value: &Uint8) -> Self {
            value.clone()
        }
    }

    impl From<u8> for Uint8 {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Uint8 {
        type Err = <u8 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Uint8 {
        type Error = <u8 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Uint8 {
        type Error = <u8 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Uint8 {
        type Error = <u8 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Uint8 {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateStrategy {
        #[serde(
            rename = "AutoPromote",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub auto_promote: Option<bool>,
        #[serde(
            rename = "AutoRevert",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub auto_revert: Option<bool>,
        #[serde(rename = "Canary", default, skip_serializing_if = "Option::is_none")]
        pub canary: Option<i64>,
        #[serde(
            rename = "HealthCheck",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub health_check: Option<String>,
        #[serde(
            rename = "HealthyDeadline",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub healthy_deadline: Option<i64>,
        #[serde(
            rename = "MaxParallel",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_parallel: Option<i64>,
        #[serde(
            rename = "MinHealthyTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub min_healthy_time: Option<i64>,
        #[serde(
            rename = "ProgressDeadline",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub progress_deadline: Option<i64>,
        #[serde(rename = "Stagger", default, skip_serializing_if = "Option::is_none")]
        pub stagger: Option<i64>,
    }

    impl From<&UpdateStrategy> for UpdateStrategy {
        fn from(value: &UpdateStrategy) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Variable {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "CreateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_time: Option<i64>,
        #[serde(rename = "Items", default, skip_serializing_if = "Option::is_none")]
        pub items: Option<VariableItems>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "ModifyTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_time: Option<i64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
    }

    impl From<&Variable> for Variable {
        fn from(value: &Variable) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct VariableItems(pub std::collections::HashMap<String, String>);
    impl std::ops::Deref for VariableItems {
        type Target = std::collections::HashMap<String, String>;
        fn deref(&self) -> &std::collections::HashMap<String, String> {
            &self.0
        }
    }

    impl From<VariableItems> for std::collections::HashMap<String, String> {
        fn from(value: VariableItems) -> Self {
            value.0
        }
    }

    impl From<&VariableItems> for VariableItems {
        fn from(value: &VariableItems) -> Self {
            value.clone()
        }
    }

    impl From<std::collections::HashMap<String, String>> for VariableItems {
        fn from(value: std::collections::HashMap<String, String>) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct VariableMetadata {
        #[serde(
            rename = "CreateIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_index: Option<u64>,
        #[serde(
            rename = "CreateTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub create_time: Option<i64>,
        #[serde(
            rename = "ModifyIndex",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_index: Option<u64>,
        #[serde(
            rename = "ModifyTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub modify_time: Option<i64>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
    }

    impl From<&VariableMetadata> for VariableMetadata {
        fn from(value: &VariableMetadata) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Vault {
        #[serde(
            rename = "ChangeMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub change_mode: Option<String>,
        #[serde(
            rename = "ChangeSignal",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub change_signal: Option<String>,
        #[serde(rename = "Env", default, skip_serializing_if = "Option::is_none")]
        pub env: Option<bool>,
        #[serde(rename = "Namespace", default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
        #[serde(rename = "Policies", default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<String>,
    }

    impl From<&Vault> for Vault {
        fn from(value: &Vault) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct VolumeMount {
        #[serde(
            rename = "Destination",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub destination: Option<String>,
        #[serde(
            rename = "PropagationMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub propagation_mode: Option<String>,
        #[serde(rename = "ReadOnly", default, skip_serializing_if = "Option::is_none")]
        pub read_only: Option<bool>,
        #[serde(rename = "Volume", default, skip_serializing_if = "Option::is_none")]
        pub volume: Option<String>,
    }

    impl From<&VolumeMount> for VolumeMount {
        fn from(value: &VolumeMount) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct VolumeRequest {
        #[serde(
            rename = "AccessMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_mode: Option<String>,
        #[serde(
            rename = "AttachmentMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attachment_mode: Option<String>,
        #[serde(
            rename = "MountOptions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mount_options: Option<CsiMountOptions>,
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "PerAlloc", default, skip_serializing_if = "Option::is_none")]
        pub per_alloc: Option<bool>,
        #[serde(rename = "ReadOnly", default, skip_serializing_if = "Option::is_none")]
        pub read_only: Option<bool>,
        #[serde(rename = "Source", default, skip_serializing_if = "Option::is_none")]
        pub source: Option<String>,
        #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&VolumeRequest> for VolumeRequest {
        fn from(value: &VolumeRequest) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WaitConfig {
        #[serde(rename = "Max", default, skip_serializing_if = "Option::is_none")]
        pub max: Option<i64>,
        #[serde(rename = "Min", default, skip_serializing_if = "Option::is_none")]
        pub min: Option<i64>,
    }

    impl From<&WaitConfig> for WaitConfig {
        fn from(value: &WaitConfig) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
///Client for Nomad
///
///Version: 1.1.4
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.1.4"
    }
}

impl Client {
    ///Sends a `POST` request to `/acl/bootstrap`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn post_acl_bootstrap<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::AclToken>, Error<()>> {
        let url = format!("{}/acl/bootstrap", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/acl/policies`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_acl_policies<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::AclPolicyListStub>>, Error<()>> {
        let url = format!("{}/acl/policies", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/acl/policy/{policyName}`
    ///
    ///Arguments:
    /// - `policy_name`: The ACL policy name.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_acl_policy<'a>(
        &'a self,
        policy_name: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::AclPolicy>, Error<()>> {
        let url = format!(
            "{}/acl/policy/{}",
            self.baseurl,
            encode_path(&policy_name.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/acl/policy/{policyName}`
    ///
    ///Arguments:
    /// - `policy_name`: The ACL policy name.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_acl_policy<'a>(
        &'a self,
        policy_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::AclPolicy,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/acl/policy/{}",
            self.baseurl,
            encode_path(&policy_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/acl/policy/{policyName}`
    ///
    ///Arguments:
    /// - `policy_name`: The ACL policy name.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn delete_acl_policy<'a>(
        &'a self,
        policy_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/acl/policy/{}",
            self.baseurl,
            encode_path(&policy_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .delete(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/acl/token`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_acl_token_self<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::AclToken>, Error<()>> {
        let url = format!("{}/acl/token", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/acl/token/{tokenAccessor}`
    ///
    ///Arguments:
    /// - `token_accessor`: The token accessor ID.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_acl_token<'a>(
        &'a self,
        token_accessor: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::AclToken>, Error<()>> {
        let url = format!(
            "{}/acl/token/{}",
            self.baseurl,
            encode_path(&token_accessor.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/acl/token/{tokenAccessor}`
    ///
    ///Arguments:
    /// - `token_accessor`: The token accessor ID.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_acl_token<'a>(
        &'a self,
        token_accessor: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::AclToken,
    ) -> Result<ResponseValue<types::AclToken>, Error<()>> {
        let url = format!(
            "{}/acl/token/{}",
            self.baseurl,
            encode_path(&token_accessor.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/acl/token/{tokenAccessor}`
    ///
    ///Arguments:
    /// - `token_accessor`: The token accessor ID.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn delete_acl_token<'a>(
        &'a self,
        token_accessor: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/acl/token/{}",
            self.baseurl,
            encode_path(&token_accessor.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .delete(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/acl/token/onetime`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn post_acl_token_onetime<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::OneTimeToken>, Error<()>> {
        let url = format!("{}/acl/token/onetime", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/acl/token/onetime/exchange`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_acl_token_onetime_exchange<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::OneTimeTokenExchangeRequest,
    ) -> Result<ResponseValue<types::AclToken>, Error<()>> {
        let url = format!("{}/acl/token/onetime/exchange", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/acl/tokens`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_acl_tokens<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::AclTokenListStub>>, Error<()>> {
        let url = format!("{}/acl/tokens", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/allocation/{allocID}`
    ///
    ///Arguments:
    /// - `alloc_id`: Allocation ID.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_allocation<'a>(
        &'a self,
        alloc_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::Allocation>, Error<()>> {
        let url = format!(
            "{}/allocation/{}",
            self.baseurl,
            encode_path(&alloc_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/allocation/{allocID}/services`
    ///
    ///Arguments:
    /// - `alloc_id`: Allocation ID.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_allocation_services<'a>(
        &'a self,
        alloc_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::ServiceRegistration>>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/services",
            self.baseurl,
            encode_path(&alloc_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/allocation/{allocID}/stop`
    ///
    ///Arguments:
    /// - `alloc_id`: Allocation ID.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `no_shutdown_delay`: Flag indicating whether to delay shutdown when
    ///   requesting an allocation stop.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn post_allocation_stop<'a>(
        &'a self,
        alloc_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        no_shutdown_delay: Option<bool>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::AllocStopResponse>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/stop",
            self.baseurl,
            encode_path(&alloc_id.to_string()),
        );
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &no_shutdown_delay {
            query.push(("no_shutdown_delay", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/allocations`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `resources`: Flag indicating whether to include resources in response.
    /// - `stale`: If present, results will include stale reads.
    /// - `task_states`: Flag indicating whether to include task states in
    ///   response.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_allocations<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        resources: Option<bool>,
        stale: Option<&'a str>,
        task_states: Option<bool>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::AllocationListStub>>, Error<()>> {
        let url = format!("{}/allocations", self.baseurl,);
        let mut query = Vec::with_capacity(9usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &resources {
            query.push(("resources", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &task_states {
            query.push(("task_states", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/deployment/{deploymentID}`
    ///
    ///Arguments:
    /// - `deployment_id`: Deployment ID.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_deployment<'a>(
        &'a self,
        deployment_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::Deployment>, Error<()>> {
        let url = format!(
            "{}/deployment/{}",
            self.baseurl,
            encode_path(&deployment_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/deployment/allocation-health/{deploymentID}`
    ///
    ///Arguments:
    /// - `deployment_id`: Deployment ID.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_deployment_allocation_health<'a>(
        &'a self,
        deployment_id: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::DeploymentAllocHealthRequest,
    ) -> Result<ResponseValue<types::DeploymentUpdateResponse>, Error<()>> {
        let url = format!(
            "{}/deployment/allocation-health/{}",
            self.baseurl,
            encode_path(&deployment_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/deployment/allocations/{deploymentID}`
    ///
    ///Arguments:
    /// - `deployment_id`: Deployment ID.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_deployment_allocations<'a>(
        &'a self,
        deployment_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::AllocationListStub>>, Error<()>> {
        let url = format!(
            "{}/deployment/allocations/{}",
            self.baseurl,
            encode_path(&deployment_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/deployment/fail/{deploymentID}`
    ///
    ///Arguments:
    /// - `deployment_id`: Deployment ID.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn post_deployment_fail<'a>(
        &'a self,
        deployment_id: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::DeploymentUpdateResponse>, Error<()>> {
        let url = format!(
            "{}/deployment/fail/{}",
            self.baseurl,
            encode_path(&deployment_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/deployment/pause/{deploymentID}`
    ///
    ///Arguments:
    /// - `deployment_id`: Deployment ID.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_deployment_pause<'a>(
        &'a self,
        deployment_id: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::DeploymentPauseRequest,
    ) -> Result<ResponseValue<types::DeploymentUpdateResponse>, Error<()>> {
        let url = format!(
            "{}/deployment/pause/{}",
            self.baseurl,
            encode_path(&deployment_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/deployment/promote/{deploymentID}`
    ///
    ///Arguments:
    /// - `deployment_id`: Deployment ID.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_deployment_promote<'a>(
        &'a self,
        deployment_id: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::DeploymentPromoteRequest,
    ) -> Result<ResponseValue<types::DeploymentUpdateResponse>, Error<()>> {
        let url = format!(
            "{}/deployment/promote/{}",
            self.baseurl,
            encode_path(&deployment_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/deployment/unblock/{deploymentID}`
    ///
    ///Arguments:
    /// - `deployment_id`: Deployment ID.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_deployment_unblock<'a>(
        &'a self,
        deployment_id: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::DeploymentUnblockRequest,
    ) -> Result<ResponseValue<types::DeploymentUpdateResponse>, Error<()>> {
        let url = format!(
            "{}/deployment/unblock/{}",
            self.baseurl,
            encode_path(&deployment_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/deployments`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_deployments<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::Deployment>>, Error<()>> {
        let url = format!("{}/deployments", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/evaluation/{evalID}`
    ///
    ///Arguments:
    /// - `eval_id`: Evaluation ID.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_evaluation<'a>(
        &'a self,
        eval_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::Evaluation>, Error<()>> {
        let url = format!(
            "{}/evaluation/{}",
            self.baseurl,
            encode_path(&eval_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/evaluation/{evalID}/allocations`
    ///
    ///Arguments:
    /// - `eval_id`: Evaluation ID.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_evaluation_allocations<'a>(
        &'a self,
        eval_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::AllocationListStub>>, Error<()>> {
        let url = format!(
            "{}/evaluation/{}/allocations",
            self.baseurl,
            encode_path(&eval_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/evaluations`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_evaluations<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::Evaluation>>, Error<()>> {
        let url = format!("{}/evaluations", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/job/{jobName}`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_job<'a>(
        &'a self,
        job_name: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::Job>, Error<()>> {
        let url = format!(
            "{}/job/{}",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/job/{jobName}`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_job<'a>(
        &'a self,
        job_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::JobRegisterRequest,
    ) -> Result<ResponseValue<types::JobRegisterResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/job/{jobName}`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `global`: Boolean flag indicating whether the operation should apply
    ///   to all instances of the job globally.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `purge`: Boolean flag indicating whether to purge allocations of the
    ///   job after deleting.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn delete_job<'a>(
        &'a self,
        job_name: &'a str,
        global: Option<bool>,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        purge: Option<bool>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::JobDeregisterResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(5usize);
        if let Some(v) = &global {
            query.push(("global", v.to_string()));
        }

        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &purge {
            query.push(("purge", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/job/{jobName}/allocations`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `all`: Specifies whether the list of allocations should include
    ///   allocations from a previously registered job with the same ID. This is
    ///   possible if the job is deregistered and reregistered.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_job_allocations<'a>(
        &'a self,
        job_name: &'a str,
        all: Option<bool>,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::AllocationListStub>>, Error<()>> {
        let url = format!(
            "{}/job/{}/allocations",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &all {
            query.push(("all", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/job/{jobName}/deployment`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_job_deployment<'a>(
        &'a self,
        job_name: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::Deployment>, Error<()>> {
        let url = format!(
            "{}/job/{}/deployment",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/job/{jobName}/deployments`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `all`: Flag indicating whether to constrain by job creation index or
    ///   not.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_job_deployments<'a>(
        &'a self,
        job_name: &'a str,
        all: Option<i64>,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::Deployment>>, Error<()>> {
        let url = format!(
            "{}/job/{}/deployments",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &all {
            query.push(("all", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/job/{jobName}/dispatch`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_job_dispatch<'a>(
        &'a self,
        job_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::JobDispatchRequest,
    ) -> Result<ResponseValue<types::JobDispatchResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}/dispatch",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/job/{jobName}/evaluate`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_job_evaluate<'a>(
        &'a self,
        job_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::JobEvaluateRequest,
    ) -> Result<ResponseValue<types::JobRegisterResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}/evaluate",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/job/{jobName}/evaluations`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_job_evaluations<'a>(
        &'a self,
        job_name: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::Evaluation>>, Error<()>> {
        let url = format!(
            "{}/job/{}/evaluations",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/job/{jobName}/periodic/force`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn post_job_periodic_force<'a>(
        &'a self,
        job_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::PeriodicForceResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}/periodic/force",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/job/{jobName}/plan`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_job_plan<'a>(
        &'a self,
        job_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::JobPlanRequest,
    ) -> Result<ResponseValue<types::JobPlanResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}/plan",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/job/{jobName}/revert`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_job_revert<'a>(
        &'a self,
        job_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::JobRevertRequest,
    ) -> Result<ResponseValue<types::JobRegisterResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}/revert",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/job/{jobName}/scale`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_job_scale_status<'a>(
        &'a self,
        job_name: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::JobScaleStatusResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}/scale",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/job/{jobName}/scale`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_job_scaling_request<'a>(
        &'a self,
        job_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::ScalingRequest,
    ) -> Result<ResponseValue<types::JobRegisterResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}/scale",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/job/{jobName}/stable`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_job_stability<'a>(
        &'a self,
        job_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::JobStabilityRequest,
    ) -> Result<ResponseValue<types::JobStabilityResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}/stable",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/job/{jobName}/summary`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_job_summary<'a>(
        &'a self,
        job_name: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::JobSummary>, Error<()>> {
        let url = format!(
            "{}/job/{}/summary",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/job/{jobName}/versions`
    ///
    ///Arguments:
    /// - `job_name`: The job identifier.
    /// - `diffs`: Boolean flag indicating whether to compute job diffs.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_job_versions<'a>(
        &'a self,
        job_name: &'a str,
        diffs: Option<bool>,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::JobVersionsResponse>, Error<()>> {
        let url = format!(
            "{}/job/{}/versions",
            self.baseurl,
            encode_path(&job_name.to_string()),
        );
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &diffs {
            query.push(("diffs", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/jobs`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_jobs<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::JobListStub>>, Error<()>> {
        let url = format!("{}/jobs", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/jobs`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn register_job<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::JobRegisterRequest,
    ) -> Result<ResponseValue<types::JobRegisterResponse>, Error<()>> {
        let url = format!("{}/jobs", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/jobs/parse`
    pub async fn post_job_parse<'a>(
        &'a self,
        body: &'a types::JobsParseRequest,
    ) -> Result<ResponseValue<types::Job>, Error<()>> {
        let url = format!("{}/jobs/parse", self.baseurl,);
        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/metrics`
    ///
    ///Arguments:
    /// - `format`: The format the user requested for the metrics summary (e.g.
    ///   prometheus)
    pub async fn get_metrics_summary<'a>(
        &'a self,
        format: Option<&'a str>,
    ) -> Result<ResponseValue<types::MetricsSummary>, Error<()>> {
        let url = format!("{}/metrics", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &format {
            query.push(("format", v.to_string()));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/namespace`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn create_namespace<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/namespace", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/namespace/{namespaceName}`
    ///
    ///Arguments:
    /// - `namespace_name`: The namespace identifier.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_namespace<'a>(
        &'a self,
        namespace_name: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::Namespace>, Error<()>> {
        let url = format!(
            "{}/namespace/{}",
            self.baseurl,
            encode_path(&namespace_name.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/namespace/{namespaceName}`
    ///
    ///Arguments:
    /// - `namespace_name`: The namespace identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_namespace<'a>(
        &'a self,
        namespace_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::Namespace,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/namespace/{}",
            self.baseurl,
            encode_path(&namespace_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/namespace/{namespaceName}`
    ///
    ///Arguments:
    /// - `namespace_name`: The namespace identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn delete_namespace<'a>(
        &'a self,
        namespace_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/namespace/{}",
            self.baseurl,
            encode_path(&namespace_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .delete(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/namespaces`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_namespaces<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::Namespace>>, Error<()>> {
        let url = format!("{}/namespaces", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/node/{nodeId}`
    ///
    ///Arguments:
    /// - `node_id`: The ID of the node.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_node<'a>(
        &'a self,
        node_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::Node>, Error<()>> {
        let url = format!(
            "{}/node/{}",
            self.baseurl,
            encode_path(&node_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/node/{nodeId}/allocations`
    ///
    ///Arguments:
    /// - `node_id`: The ID of the node.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_node_allocations<'a>(
        &'a self,
        node_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::AllocationListStub>>, Error<()>> {
        let url = format!(
            "{}/node/{}/allocations",
            self.baseurl,
            encode_path(&node_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/node/{nodeId}/drain`
    ///
    ///Arguments:
    /// - `node_id`: The ID of the node.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    /// - `body`
    pub async fn update_node_drain<'a>(
        &'a self,
        node_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
        body: &'a types::NodeUpdateDrainRequest,
    ) -> Result<ResponseValue<types::NodeDrainUpdateResponse>, Error<()>> {
        let url = format!(
            "{}/node/{}/drain",
            self.baseurl,
            encode_path(&node_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/node/{nodeId}/eligibility`
    ///
    ///Arguments:
    /// - `node_id`: The ID of the node.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    /// - `body`
    pub async fn update_node_eligibility<'a>(
        &'a self,
        node_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
        body: &'a types::NodeUpdateEligibilityRequest,
    ) -> Result<ResponseValue<types::NodeEligibilityUpdateResponse>, Error<()>> {
        let url = format!(
            "{}/node/{}/eligibility",
            self.baseurl,
            encode_path(&node_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/node/{nodeId}/purge`
    ///
    ///Arguments:
    /// - `node_id`: The ID of the node.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn update_node_purge<'a>(
        &'a self,
        node_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::NodePurgeResponse>, Error<()>> {
        let url = format!(
            "{}/node/{}/purge",
            self.baseurl,
            encode_path(&node_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/nodes`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `resources`: Whether or not to include the NodeResources and
    ///   ReservedResources fields in the response.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_nodes<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        resources: Option<bool>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::NodeListStub>>, Error<()>> {
        let url = format!("{}/nodes", self.baseurl,);
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &resources {
            query.push(("resources", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/operator/autopilot/configuration`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_operator_autopilot_configuration<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::AutopilotConfiguration>, Error<()>> {
        let url = format!("{}/operator/autopilot/configuration", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/operator/autopilot/configuration`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn put_operator_autopilot_configuration<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::AutopilotConfiguration,
    ) -> Result<ResponseValue<bool>, Error<()>> {
        let url = format!("{}/operator/autopilot/configuration", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/operator/autopilot/health`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_operator_autopilot_health<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::OperatorHealthReply>, Error<()>> {
        let url = format!("{}/operator/autopilot/health", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/operator/raft/configuration`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_operator_raft_configuration<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::RaftConfiguration>, Error<()>> {
        let url = format!("{}/operator/raft/configuration", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/operator/raft/peer`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn delete_operator_raft_peer<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/operator/raft/peer", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .delete(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/operator/scheduler/configuration`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_operator_scheduler_configuration<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::SchedulerConfigurationResponse>, Error<()>> {
        let url = format!("{}/operator/scheduler/configuration", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/operator/scheduler/configuration`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_operator_scheduler_configuration<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::SchedulerConfiguration,
    ) -> Result<ResponseValue<types::SchedulerSetConfigurationResponse>, Error<()>> {
        let url = format!("{}/operator/scheduler/configuration", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/plugin/csi/{pluginID}`
    ///
    ///Arguments:
    /// - `plugin_id`: The CSI plugin identifier.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_plugin_csi<'a>(
        &'a self,
        plugin_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::CsiPlugin>>, Error<()>> {
        let url = format!(
            "{}/plugin/csi/{}",
            self.baseurl,
            encode_path(&plugin_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/plugins`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_plugins<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::CsiPluginListStub>>, Error<()>> {
        let url = format!("{}/plugins", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/quota`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn create_quota_spec<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::QuotaSpec,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/quota", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/quota/{specName}`
    ///
    ///Arguments:
    /// - `spec_name`: The quota spec identifier.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_quota_spec<'a>(
        &'a self,
        spec_name: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::QuotaSpec>, Error<()>> {
        let url = format!(
            "{}/quota/{}",
            self.baseurl,
            encode_path(&spec_name.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/quota/{specName}`
    ///
    ///Arguments:
    /// - `spec_name`: The quota spec identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_quota_spec<'a>(
        &'a self,
        spec_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::QuotaSpec,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/quota/{}",
            self.baseurl,
            encode_path(&spec_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/quota/{specName}`
    ///
    ///Arguments:
    /// - `spec_name`: The quota spec identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn delete_quota_spec<'a>(
        &'a self,
        spec_name: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/quota/{}",
            self.baseurl,
            encode_path(&spec_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .delete(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/quotas`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_quotas<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<serde_json::Value>>, Error<()>> {
        let url = format!("{}/quotas", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/regions`
    pub async fn get_regions<'a>(&'a self) -> Result<ResponseValue<Vec<String>>, Error<()>> {
        let url = format!("{}/regions", self.baseurl,);
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/scaling/policies`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_scaling_policies<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::ScalingPolicyListStub>>, Error<()>> {
        let url = format!("{}/scaling/policies", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/scaling/policy/{policyID}`
    ///
    ///Arguments:
    /// - `policy_id`: The scaling policy identifier.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_scaling_policy<'a>(
        &'a self,
        policy_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::ScalingPolicy>, Error<()>> {
        let url = format!(
            "{}/scaling/policy/{}",
            self.baseurl,
            encode_path(&policy_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/search`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    /// - `body`
    pub async fn get_search<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
        body: &'a types::SearchRequest,
    ) -> Result<ResponseValue<types::SearchResponse>, Error<()>> {
        let url = format!("{}/search", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/search/fuzzy`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    /// - `body`
    pub async fn get_fuzzy_search<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
        body: &'a types::FuzzySearchRequest,
    ) -> Result<ResponseValue<types::FuzzySearchResponse>, Error<()>> {
        let url = format!("{}/search/fuzzy", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/status/leader`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_status_leader<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<String>, Error<()>> {
        let url = format!("{}/status/leader", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/status/peers`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_status_peers<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<String>>, Error<()>> {
        let url = format!("{}/status/peers", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/system/gc`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn put_system_gc<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/system/gc", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .put(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/system/reconcile/summaries`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn put_system_reconcile_summaries<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/system/reconcile/summaries", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .put(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/validate/job`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_job_validate_request<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::JobValidateRequest,
    ) -> Result<ResponseValue<types::JobValidateResponse>, Error<()>> {
        let url = format!("{}/validate/job", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/var/{path}`
    ///
    ///Arguments:
    /// - `path`: A path to a Nomad Variable
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_variable_query<'a>(
        &'a self,
        path: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::Variable>, Error<()>> {
        let url = format!("{}/var/{}", self.baseurl, encode_path(&path.to_string()),);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/var/{path}`
    ///
    ///Arguments:
    /// - `path`: A path to a Nomad Variable
    /// - `cas`: A compare-and-set parameter for Nomad Variables
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn put_variable<'a>(
        &'a self,
        path: &'a str,
        cas: Option<i64>,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::Variable,
    ) -> Result<ResponseValue<types::Variable>, Error<()>> {
        let url = format!("{}/var/{}", self.baseurl, encode_path(&path.to_string()),);
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &cas {
            query.push(("cas", v.to_string()));
        }

        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/var/{path}`
    ///
    ///Arguments:
    /// - `path`: A path to a Nomad Variable
    /// - `cas`: A compare-and-set parameter for Nomad Variables
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_variable<'a>(
        &'a self,
        path: &'a str,
        cas: Option<i64>,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::Variable,
    ) -> Result<ResponseValue<types::Variable>, Error<()>> {
        let url = format!("{}/var/{}", self.baseurl, encode_path(&path.to_string()),);
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &cas {
            query.push(("cas", v.to_string()));
        }

        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/var/{path}`
    ///
    ///Arguments:
    /// - `path`: A path to a Nomad Variable
    /// - `cas`: A compare-and-set parameter for Nomad Variables
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn delete_variable<'a>(
        &'a self,
        path: &'a str,
        cas: Option<i64>,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::Variable,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/var/{}", self.baseurl, encode_path(&path.to_string()),);
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &cas {
            query.push(("cas", v.to_string()));
        }

        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .delete(url)
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/vars`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_variables_list_request<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::VariableMetadata>>, Error<()>> {
        let url = format!("{}/vars", self.baseurl,);
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/volume/csi/{volumeId}`
    ///
    ///Arguments:
    /// - `volume_id`: Volume unique identifier.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_volume<'a>(
        &'a self,
        volume_id: &'a str,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::CsiVolume>, Error<()>> {
        let url = format!(
            "{}/volume/csi/{}",
            self.baseurl,
            encode_path(&volume_id.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/volume/csi/{volumeId}`
    ///
    ///Arguments:
    /// - `volume_id`: Volume unique identifier.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_volume_registration<'a>(
        &'a self,
        volume_id: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::CsiVolumeRegisterRequest,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/volume/csi/{}",
            self.baseurl,
            encode_path(&volume_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/volume/csi/{volumeId}`
    ///
    ///Arguments:
    /// - `volume_id`: Volume unique identifier.
    /// - `force`: Used to force the de-registration of a volume.
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn delete_volume_registration<'a>(
        &'a self,
        volume_id: &'a str,
        force: Option<&'a str>,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/volume/csi/{}",
            self.baseurl,
            encode_path(&volume_id.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &force {
            query.push(("force", v.to_string()));
        }

        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .delete(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/volume/csi/{volumeId}/{action}`
    ///
    ///Arguments:
    /// - `volume_id`: Volume unique identifier.
    /// - `action`: The action to perform on the Volume (create, detach,
    ///   delete).
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn create_volume<'a>(
        &'a self,
        volume_id: &'a str,
        action: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::CsiVolumeCreateRequest,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/volume/csi/{}/{}",
            self.baseurl,
            encode_path(&volume_id.to_string()),
            encode_path(&action.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/volume/csi/{volumeId}/{action}`
    ///
    ///Arguments:
    /// - `volume_id`: Volume unique identifier.
    /// - `action`: The action to perform on the Volume (create, detach,
    ///   delete).
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `node`: Specifies node to target volume operation for.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn detach_or_delete_volume<'a>(
        &'a self,
        volume_id: &'a str,
        action: &'a str,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        node: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/volume/csi/{}/{}",
            self.baseurl,
            encode_path(&volume_id.to_string()),
            encode_path(&action.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &node {
            query.push(("node", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .delete(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/volumes`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `node_id`: Filters volume lists by node ID.
    /// - `per_page`: Maximum number of results to return.
    /// - `plugin_id`: Filters volume lists by plugin ID.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `type_`: Filters volume lists to a specific type.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_volumes<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        node_id: Option<&'a str>,
        per_page: Option<i64>,
        plugin_id: Option<&'a str>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        type_: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<Vec<types::CsiVolumeListStub>>, Error<()>> {
        let url = format!("{}/volumes", self.baseurl,);
        let mut query = Vec::with_capacity(10usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &node_id {
            query.push(("node_id", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &plugin_id {
            query.push(("plugin_id", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/volumes`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_volume<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::CsiVolumeRegisterRequest,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/volumes", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/volumes/external`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `plugin_id`: Filters volume lists by plugin ID.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_external_volumes<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        plugin_id: Option<&'a str>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::CsiVolumeListExternalResponse>, Error<()>> {
        let url = format!("{}/volumes/external", self.baseurl,);
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &plugin_id {
            query.push(("plugin_id", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/volumes/snapshot`
    ///
    ///Arguments:
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `next_token`: Indicates where to start paging for queries that support
    ///   pagination.
    /// - `per_page`: Maximum number of results to return.
    /// - `plugin_id`: Filters volume lists by plugin ID.
    /// - `prefix`: Constrains results to jobs that start with the defined
    ///   prefix
    /// - `region`: Filters results based on the specified region.
    /// - `stale`: If present, results will include stale reads.
    /// - `wait`: Provided with IndexParam to wait for change.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `index`: If set, wait until query exceeds given index. Must be
    ///   provided with WaitParam.
    pub async fn get_snapshots<'a>(
        &'a self,
        namespace: Option<&'a str>,
        next_token: Option<&'a str>,
        per_page: Option<i64>,
        plugin_id: Option<&'a str>,
        prefix: Option<&'a str>,
        region: Option<&'a str>,
        stale: Option<&'a str>,
        wait: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        index: Option<i64>,
    ) -> Result<ResponseValue<types::CsiSnapshotListResponse>, Error<()>> {
        let url = format!("{}/volumes/snapshot", self.baseurl,);
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &next_token {
            query.push(("next_token", v.to_string()));
        }

        if let Some(v) = &per_page {
            query.push(("per_page", v.to_string()));
        }

        if let Some(v) = &plugin_id {
            query.push(("plugin_id", v.to_string()));
        }

        if let Some(v) = &prefix {
            query.push(("prefix", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &stale {
            query.push(("stale", v.to_string()));
        }

        if let Some(v) = &wait {
            query.push(("wait", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(2usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        if let Some(v) = index {
            header_map.append("index", HeaderValue::from(v));
        }

        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/volumes/snapshot`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `region`: Filters results based on the specified region.
    /// - `x_nomad_token`: A Nomad ACL token.
    /// - `body`
    pub async fn post_snapshot<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        region: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
        body: &'a types::CsiSnapshotCreateRequest,
    ) -> Result<ResponseValue<types::CsiSnapshotCreateResponse>, Error<()>> {
        let url = format!("{}/volumes/snapshot", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/volumes/snapshot`
    ///
    ///Arguments:
    /// - `idempotency_token`: Can be used to ensure operations are only run
    ///   once.
    /// - `namespace`: Filters results based on the specified namespace.
    /// - `plugin_id`: Filters volume lists by plugin ID.
    /// - `region`: Filters results based on the specified region.
    /// - `snapshot_id`: The ID of the snapshot to target.
    /// - `x_nomad_token`: A Nomad ACL token.
    pub async fn delete_snapshot<'a>(
        &'a self,
        idempotency_token: Option<&'a str>,
        namespace: Option<&'a str>,
        plugin_id: Option<&'a str>,
        region: Option<&'a str>,
        snapshot_id: Option<&'a str>,
        x_nomad_token: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/volumes/snapshot", self.baseurl,);
        let mut query = Vec::with_capacity(5usize);
        if let Some(v) = &idempotency_token {
            query.push(("idempotency_token", v.to_string()));
        }

        if let Some(v) = &namespace {
            query.push(("namespace", v.to_string()));
        }

        if let Some(v) = &plugin_id {
            query.push(("plugin_id", v.to_string()));
        }

        if let Some(v) = &region {
            query.push(("region", v.to_string()));
        }

        if let Some(v) = &snapshot_id {
            query.push(("snapshot_id", v.to_string()));
        }

        let mut header_map = HeaderMap::with_capacity(1usize);
        if let Some(v) = x_nomad_token {
            header_map.append("X-Nomad-Token", HeaderValue::try_from(v)?);
        }

        let request = self
            .client
            .delete(url)
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            405u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

pub mod prelude {
    pub use super::Client;
}
