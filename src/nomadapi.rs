use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
  #[serde(
    rename(deserialize = "region", serialize = "Region"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub region: Option<String>,
  #[serde(
    rename(deserialize = "namespace", serialize = "Namespace"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub namespace: Option<String>,
  #[serde(
    rename(deserialize = "id", serialize = "ID"),
    alias = "__label__",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub id: Option<String>,
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub name: Option<String>,
  #[serde(
    rename(deserialize = "type", serialize = "Type"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub r#type: Option<String>,
  #[serde(
    rename(deserialize = "priority", serialize = "Priority"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub priority: Option<isize>,
  #[serde(
    rename(deserialize = "all_at_once", serialize = "AllAtOnce"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub all_at_once: Option<bool>,
  #[serde(
    rename(deserialize = "datacenters", serialize = "Datacenters"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub datacenters: Option<Vec<String>>,
  #[serde(
    rename(deserialize = "node_pool", serialize = "NodePool"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub node_pool: Option<String>,
  #[serde(
    rename(deserialize = "constraint", serialize = "Constraints"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub constraint: Option<Vec<Constraint>>,
  #[serde(
    rename(deserialize = "affinity", serialize = "Affinities"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub affinity: Option<Vec<Affinity>>,
  #[serde(
    rename(deserialize = "group", serialize = "TaskGroups"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub group: Option<Vec<TaskGroup>>,
  #[serde(
    rename(deserialize = "update", serialize = "Update"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub update: Option<UpdateStrategy>,
  #[serde(
    rename(deserialize = "multiregion", serialize = "Multiregion"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub multiregion: Option<Multiregion>,
  #[serde(
    rename(deserialize = "spread", serialize = "Spreads"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub spread: Option<Vec<Spread>>,
  #[serde(
    rename(deserialize = "periodic", serialize = "Periodic"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub periodic: Option<PeriodicConfig>,
  #[serde(
    rename(deserialize = "parameterized", serialize = "ParameterizedJob"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub parameterized: Option<ParameterizedJobConfig>,
  #[serde(
    rename(deserialize = "reschedule", serialize = "Reschedule"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub reschedule: Option<ReschedulePolicy>,
  #[serde(
    rename(deserialize = "migrate", serialize = "Migrate"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub migrate: Option<MigrateStrategy>,
  #[serde(
    rename(deserialize = "meta", serialize = "Meta"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub meta: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "consul_token", serialize = "ConsulToken"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub consul_token: Option<String>,
  #[serde(
    rename(deserialize = "vault_token", serialize = "VaultToken"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub vault_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Constraint {
  #[serde(
    rename(deserialize = "attribute", serialize = "LTarget"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub attribute: Option<String>,
  #[serde(
    rename(deserialize = "value", serialize = "RTarget"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub value: Option<String>,
  #[serde(
    rename(deserialize = "operator", serialize = "Operand"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub operator: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Affinity {
  #[serde(
    rename(deserialize = "attribute", serialize = "LTarget"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub attribute: Option<String>,
  #[serde(
    rename(deserialize = "value", serialize = "RTarget"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub value: Option<String>,
  #[serde(
    rename(deserialize = "operator", serialize = "Operand"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub operator: Option<String>,
  #[serde(
    rename(deserialize = "weight", serialize = "Weight"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub weight: Option<i8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TaskGroup {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    alias = "__label__",
    default
  )]
  pub name: String,
  #[serde(
    rename(deserialize = "count", serialize = "Count"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub count: Option<isize>,
  #[serde(
    rename(deserialize = "constraint", serialize = "Constraints"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub constraint: Option<Vec<Constraint>>,
  #[serde(
    rename(deserialize = "affinity", serialize = "Affinities"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub affinity: Option<Vec<Affinity>>,
  #[serde(
    rename(deserialize = "task", serialize = "Tasks"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub task: Option<Vec<Task>>,
  #[serde(
    rename(deserialize = "spread", serialize = "Spreads"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub spread: Option<Vec<Spread>>,
  #[serde(
    rename(deserialize = "volume", serialize = "Volumes"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub volume: Option<IndexMap<String, VolumeRequest>>,
  #[serde(
    rename(deserialize = "restart", serialize = "RestartPolicy"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub restart: Option<RestartPolicy>,
  #[serde(
    rename(deserialize = "reschedule", serialize = "ReschedulePolicy"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub reschedule: Option<ReschedulePolicy>,
  #[serde(
    rename(deserialize = "ephemeral_disk", serialize = "EphemeralDisk"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub ephemeral_disk: Option<EphemeralDisk>,
  #[serde(
    rename(deserialize = "update", serialize = "Update"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub update: Option<UpdateStrategy>,
  #[serde(
    rename(deserialize = "migrate", serialize = "Migrate"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub migrate: Option<MigrateStrategy>,
  #[serde(
    rename(deserialize = "network", serialize = "Networks"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub network: Option<Vec<NetworkResource>>,
  #[serde(
    rename(deserialize = "meta", serialize = "Meta"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub meta: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "service", serialize = "Services"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub service: Option<Vec<Service>>,
  #[serde(
    rename(deserialize = "shutdown_delay", serialize = "ShutdownDelay"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub shutdown_delay: Option<i64>,
  #[serde(
    rename(
      deserialize = "stop_after_client_disconnect",
      serialize = "StopAfterClientDisconnect"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub stop_after_client_disconnect: Option<i64>,
  #[serde(
    rename(
      deserialize = "max_client_disconnect",
      serialize = "MaxClientDisconnect"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub max_client_disconnect: Option<i64>,
  #[serde(
    rename(deserialize = "scaling", serialize = "Scaling"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub scaling: Option<ScalingPolicy>,
  #[serde(
    rename(deserialize = "consul", serialize = "Consul"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub consul: Option<Consul>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    alias = "__label__",
    default
  )]
  pub name: String,
  #[serde(
    rename(deserialize = "driver", serialize = "Driver"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub driver: Option<String>,
  #[serde(
    rename(deserialize = "user", serialize = "User"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub user: Option<String>,
  #[serde(
    rename(deserialize = "lifecycle", serialize = "Lifecycle"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub lifecycle: Option<TaskLifecycle>,
  #[serde(
    rename(deserialize = "config", serialize = "Config"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub config: Option<IndexMap<String, serde_json::Value>>,
  #[serde(
    rename(deserialize = "constraint", serialize = "Constraints"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub constraint: Option<Vec<Constraint>>,
  #[serde(
    rename(deserialize = "affinity", serialize = "Affinities"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub affinity: Option<Vec<Affinity>>,
  #[serde(
    rename(deserialize = "env", serialize = "Env"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub env: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "service", serialize = "Services"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub service: Option<Vec<Service>>,
  #[serde(
    rename(deserialize = "resources", serialize = "Resources"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub resources: Option<Resources>,
  #[serde(
    rename(deserialize = "restart", serialize = "RestartPolicy"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub restart: Option<RestartPolicy>,
  #[serde(
    rename(deserialize = "meta", serialize = "Meta"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub meta: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "kill_timeout", serialize = "KillTimeout"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub kill_timeout: Option<i64>,
  #[serde(
    rename(deserialize = "logs", serialize = "LogConfig"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub logs: Option<LogConfig>,
  #[serde(
    rename(deserialize = "artifact", serialize = "Artifacts"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub artifact: Option<Vec<TaskArtifact>>,
  #[serde(
    rename(deserialize = "vault", serialize = "Vault"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub vault: Option<Vault>,
  #[serde(
    rename(deserialize = "template", serialize = "Templates"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub template: Option<Vec<Template>>,
  #[serde(
    rename(deserialize = "dispatch_payload", serialize = "DispatchPayload"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub dispatch_payload: Option<DispatchPayloadConfig>,
  #[serde(
    rename(deserialize = "volume_mount", serialize = "VolumeMounts"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub volume_mount: Option<Vec<VolumeMount>>,
  #[serde(
    rename(deserialize = "csi_plugin", serialize = "CSIPluginConfig"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub csi_plugin: Option<TaskCSIPluginConfig>,
  #[serde(
    rename(deserialize = "leader", serialize = "Leader"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub leader: Option<bool>,
  #[serde(
    rename(deserialize = "shutdown_delay", serialize = "ShutdownDelay"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub shutdown_delay: Option<i64>,
  #[serde(
    rename(deserialize = "kill_signal", serialize = "KillSignal"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub kill_signal: Option<String>,
  #[serde(
    rename(deserialize = "kind", serialize = "Kind"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub kind: Option<String>,
  #[serde(
    rename(deserialize = "scaling", serialize = "ScalingPolicies"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub scaling: Option<Vec<ScalingPolicy>>,
  #[serde(
    rename(deserialize = "identity", serialize = "Identity"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub identity: Option<WorkloadIdentity>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TaskLifecycle {
  #[serde(
    rename(deserialize = "hook", serialize = "Hook"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub hook: Option<String>,
  #[serde(
    rename(deserialize = "sidecar", serialize = "Sidecar"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub sidecar: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Service {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    alias = "__label__",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub name: Option<String>,
  #[serde(
    rename(deserialize = "tags", serialize = "Tags"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub tags: Option<Vec<String>>,
  #[serde(
    rename(deserialize = "canary_tags", serialize = "CanaryTags"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub canary_tags: Option<Vec<String>>,
  #[serde(
    rename(
      deserialize = "enable_tag_override",
      serialize = "EnableTagOverride"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub enable_tag_override: Option<bool>,
  #[serde(
    rename(deserialize = "port", serialize = "PortLabel"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub port: Option<String>,
  #[serde(
    rename(deserialize = "address_mode", serialize = "AddressMode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub address_mode: Option<String>,
  #[serde(
    rename(deserialize = "address", serialize = "Address"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub address: Option<String>,
  #[serde(
    rename(deserialize = "check", serialize = "Checks"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub check: Option<Vec<ServiceCheck>>,
  #[serde(
    rename(deserialize = "check_restart", serialize = "CheckRestart"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub check_restart: Option<CheckRestart>,
  #[serde(
    rename(deserialize = "connect", serialize = "Connect"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub connect: Option<ConsulConnect>,
  #[serde(
    rename(deserialize = "meta", serialize = "Meta"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub meta: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "canary_meta", serialize = "CanaryMeta"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub canary_meta: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "tagged_addresses", serialize = "TaggedAddresses"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub tagged_addresses: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "task", serialize = "TaskName"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub task: Option<String>,
  #[serde(
    rename(deserialize = "on_update", serialize = "OnUpdate"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub on_update: Option<String>,
  #[serde(
    rename(deserialize = "provider", serialize = "Provider"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub provider: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceCheck {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    alias = "__label__",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub name: Option<String>,
  #[serde(
    rename(deserialize = "type", serialize = "Type"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub r#type: Option<String>,
  #[serde(
    rename(deserialize = "command", serialize = "Command"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub command: Option<String>,
  #[serde(
    rename(deserialize = "args", serialize = "Args"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub args: Option<Vec<String>>,
  #[serde(
    rename(deserialize = "path", serialize = "Path"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub path: Option<String>,
  #[serde(
    rename(deserialize = "protocol", serialize = "Protocol"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub protocol: Option<String>,
  #[serde(
    rename(deserialize = "port", serialize = "PortLabel"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub port: Option<String>,
  #[serde(
    rename(deserialize = "expose", serialize = "Expose"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub expose: Option<bool>,
  #[serde(
    rename(deserialize = "address_mode", serialize = "AddressMode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub address_mode: Option<String>,
  #[serde(
    rename(deserialize = "advertise", serialize = "Advertise"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub advertise: Option<String>,
  #[serde(
    rename(deserialize = "interval", serialize = "Interval"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub interval: Option<i64>,
  #[serde(
    rename(deserialize = "timeout", serialize = "Timeout"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub timeout: Option<i64>,
  #[serde(
    rename(deserialize = "initial_status", serialize = "InitialStatus"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub initial_status: Option<String>,
  #[serde(
    rename(deserialize = "tls_server_name", serialize = "TLSServerName"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub tls_server_name: Option<String>,
  #[serde(
    rename(deserialize = "tls_skip_verify", serialize = "TLSSkipVerify"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub tls_skip_verify: Option<bool>,
  #[serde(
    rename(deserialize = "header", serialize = "Header"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub header: Option<IndexMap<String, Vec<String>>>,
  #[serde(
    rename(deserialize = "method", serialize = "Method"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub method: Option<String>,
  #[serde(
    rename(deserialize = "check_restart", serialize = "CheckRestart"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub check_restart: Option<CheckRestart>,
  #[serde(
    rename(deserialize = "grpc_service", serialize = "GRPCService"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub grpc_service: Option<String>,
  #[serde(
    rename(deserialize = "grpc_use_tls", serialize = "GRPCUseTLS"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub grpc_use_tls: Option<bool>,
  #[serde(
    rename(deserialize = "task", serialize = "TaskName"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub task: Option<String>,
  #[serde(
    rename(
      deserialize = "success_before_passing",
      serialize = "SuccessBeforePassing"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub success_before_passing: Option<isize>,
  #[serde(
    rename(
      deserialize = "failures_before_critical",
      serialize = "FailuresBeforeCritical"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub failures_before_critical: Option<isize>,
  #[serde(
    rename(deserialize = "body", serialize = "Body"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub body: Option<String>,
  #[serde(
    rename(deserialize = "on_update", serialize = "OnUpdate"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub on_update: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckRestart {
  #[serde(
    rename(deserialize = "limit", serialize = "Limit"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub limit: Option<isize>,
  #[serde(
    rename(deserialize = "grace", serialize = "Grace"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub grace: Option<i64>,
  #[serde(
    rename(deserialize = "ignore_warnings", serialize = "IgnoreWarnings"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub ignore_warnings: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulConnect {
  #[serde(
    rename(deserialize = "native", serialize = "Native"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub native: Option<bool>,
  #[serde(
    rename(deserialize = "gateway", serialize = "Gateway"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub gateway: Option<ConsulGateway>,
  #[serde(
    rename(deserialize = "sidecar_service", serialize = "SidecarService"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub sidecar_service: Option<ConsulSidecarService>,
  #[serde(
    rename(deserialize = "sidecar_task", serialize = "SidecarTask"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub sidecar_task: Option<SidecarTask>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulGateway {
  #[serde(
    rename(deserialize = "proxy", serialize = "Proxy"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub proxy: Option<ConsulGatewayProxy>,
  #[serde(
    rename(deserialize = "ingress", serialize = "Ingress"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub ingress: Option<ConsulIngressConfigEntry>,
  #[serde(
    rename(deserialize = "terminating", serialize = "Terminating"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub terminating: Option<ConsulTerminatingConfigEntry>,
  #[serde(
    rename(deserialize = "mesh", serialize = "Mesh"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub mesh: Option<ConsulMeshConfigEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulGatewayProxy {
  #[serde(
    rename(deserialize = "connect_timeout", serialize = "ConnectTimeout"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub connect_timeout: Option<i64>,
  #[serde(
    rename(
      deserialize = "envoy_gateway_bind_tagged_addresses",
      serialize = "EnvoyGatewayBindTaggedAddresses"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub envoy_gateway_bind_tagged_addresses: Option<bool>,
  #[serde(
    rename(
      deserialize = "envoy_gateway_bind_addresses",
      serialize = "EnvoyGatewayBindAddresses"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub envoy_gateway_bind_addresses:
    Option<IndexMap<String, ConsulGatewayBindAddress>>,
  #[serde(
    rename(
      deserialize = "envoy_gateway_no_default_bind",
      serialize = "EnvoyGatewayNoDefaultBind"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub envoy_gateway_no_default_bind: Option<bool>,
  #[serde(
    rename(
      deserialize = "envoy_dns_discovery_type",
      serialize = "EnvoyDNSDiscoveryType"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub envoy_dns_discovery_type: Option<String>,
  #[serde(
    rename(deserialize = "config", serialize = "Config"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub config: Option<IndexMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulGatewayBindAddress {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub name: Option<String>,
  #[serde(
    rename(deserialize = "address", serialize = "Address"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub address: Option<String>,
  #[serde(
    rename(deserialize = "port", serialize = "Port"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub port: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulIngressConfigEntry {
  #[serde(
    rename(deserialize = "tls", serialize = "TLS"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub tls: Option<ConsulGatewayTLSConfig>,
  #[serde(
    rename(deserialize = "listener", serialize = "Listeners"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub listener: Option<Vec<ConsulIngressListener>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulGatewayTLSConfig {
  #[serde(
    rename(deserialize = "enabled", serialize = "Enabled"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub enabled: Option<bool>,
  #[serde(
    rename(deserialize = "tls_min_version", serialize = "TLSMinVersion"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub tls_min_version: Option<String>,
  #[serde(
    rename(deserialize = "tls_max_version", serialize = "TLSMaxVersion"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub tls_max_version: Option<String>,
  #[serde(
    rename(deserialize = "cipher_suites", serialize = "CipherSuites"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub cipher_suites: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulIngressListener {
  #[serde(
    rename(deserialize = "port", serialize = "Port"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub port: Option<isize>,
  #[serde(
    rename(deserialize = "protocol", serialize = "Protocol"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub protocol: Option<String>,
  #[serde(
    rename(deserialize = "service", serialize = "Services"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub service: Option<Vec<ConsulIngressService>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulIngressService {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub name: Option<String>,
  #[serde(
    rename(deserialize = "hosts", serialize = "Hosts"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub hosts: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulTerminatingConfigEntry {
  #[serde(
    rename(deserialize = "service", serialize = "Services"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub service: Option<Vec<ConsulLinkedService>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulLinkedService {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub name: Option<String>,
  #[serde(
    rename(deserialize = "ca_file", serialize = "CAFile"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub ca_file: Option<String>,
  #[serde(
    rename(deserialize = "cert_file", serialize = "CertFile"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub cert_file: Option<String>,
  #[serde(
    rename(deserialize = "key_file", serialize = "KeyFile"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub key_file: Option<String>,
  #[serde(
    rename(deserialize = "sni", serialize = "SNI"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub sni: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulMeshConfigEntry {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulSidecarService {
  #[serde(
    rename(deserialize = "tags", serialize = "Tags"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub tags: Option<Vec<String>>,
  #[serde(
    rename(deserialize = "port", serialize = "Port"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub port: Option<String>,
  #[serde(
    rename(deserialize = "proxy", serialize = "Proxy"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub proxy: Option<ConsulProxy>,
  #[serde(
    rename(
      deserialize = "disable_default_tcp_check",
      serialize = "DisableDefaultTCPCheck"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub disable_default_tcp_check: Option<bool>,
  #[serde(
    rename(deserialize = "meta", serialize = "Meta"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub meta: Option<IndexMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulProxy {
  #[serde(
    rename(
      deserialize = "local_service_address",
      serialize = "LocalServiceAddress"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub local_service_address: Option<String>,
  #[serde(
    rename(deserialize = "local_service_port", serialize = "LocalServicePort"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub local_service_port: Option<isize>,
  #[serde(
    rename(deserialize = "expose", serialize = "Expose"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub expose: Option<ConsulExposeConfig>,
  #[serde(
    rename(deserialize = "upstreams", serialize = "Upstreams"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub upstreams: Option<Vec<ConsulUpstream>>,
  #[serde(
    rename(deserialize = "config", serialize = "Config"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub config: Option<IndexMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulExposeConfig {
  #[serde(
    rename(deserialize = "path", serialize = "Paths"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub path: Option<Vec<ConsulExposePath>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulExposePath {
  #[serde(
    rename(deserialize = "path", serialize = "Path"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub path: Option<String>,
  #[serde(
    rename(deserialize = "protocol", serialize = "Protocol"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub protocol: Option<String>,
  #[serde(
    rename(deserialize = "local_path_port", serialize = "LocalPathPort"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub local_path_port: Option<isize>,
  #[serde(
    rename(deserialize = "listener_port", serialize = "ListenerPort"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub listener_port: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulUpstream {
  #[serde(
    rename(deserialize = "destination_name", serialize = "DestinationName"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub destination_name: Option<String>,
  #[serde(
    rename(
      deserialize = "destination_namespace",
      serialize = "DestinationNamespace"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub destination_namespace: Option<String>,
  #[serde(
    rename(deserialize = "local_bind_port", serialize = "LocalBindPort"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub local_bind_port: Option<isize>,
  #[serde(
    rename(deserialize = "datacenter", serialize = "Datacenter"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub datacenter: Option<String>,
  #[serde(
    rename(deserialize = "local_bind_address", serialize = "LocalBindAddress"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub local_bind_address: Option<String>,
  #[serde(
    rename(deserialize = "mesh_gateway", serialize = "MeshGateway"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub mesh_gateway: Option<ConsulMeshGateway>,
  #[serde(
    rename(deserialize = "config", serialize = "Config"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub config: Option<IndexMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConsulMeshGateway {
  #[serde(
    rename(deserialize = "mode", serialize = "Mode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub mode: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SidecarTask {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub name: Option<String>,
  #[serde(
    rename(deserialize = "driver", serialize = "Driver"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub driver: Option<String>,
  #[serde(
    rename(deserialize = "user", serialize = "User"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub user: Option<String>,
  #[serde(
    rename(deserialize = "config", serialize = "Config"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub config: Option<IndexMap<String, serde_json::Value>>,
  #[serde(
    rename(deserialize = "env", serialize = "Env"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub env: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "resources", serialize = "Resources"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub resources: Option<Resources>,
  #[serde(
    rename(deserialize = "meta", serialize = "Meta"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub meta: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "kill_timeout", serialize = "KillTimeout"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub kill_timeout: Option<i64>,
  #[serde(
    rename(deserialize = "logs", serialize = "LogConfig"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub logs: Option<LogConfig>,
  #[serde(
    rename(deserialize = "shutdown_delay", serialize = "ShutdownDelay"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub shutdown_delay: Option<i64>,
  #[serde(
    rename(deserialize = "kill_signal", serialize = "KillSignal"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub kill_signal: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Resources {
  #[serde(
    rename(deserialize = "cpu", serialize = "CPU"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub cpu: Option<isize>,
  #[serde(
    rename(deserialize = "cores", serialize = "Cores"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub cores: Option<isize>,
  #[serde(
    rename(deserialize = "memory", serialize = "MemoryMB"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub memory: Option<isize>,
  #[serde(
    rename(deserialize = "memory_max", serialize = "MemoryMaxMB"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub memory_max: Option<isize>,
  #[serde(
    rename(deserialize = "disk", serialize = "DiskMB"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub disk: Option<isize>,
  #[serde(
    rename(deserialize = "network", serialize = "Networks"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub network: Option<Vec<NetworkResource>>,
  #[serde(
    rename(deserialize = "device", serialize = "Devices"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub device: Option<Vec<RequestedDevice>>,
  #[serde(
    rename(deserialize = "iops", serialize = "IOPS"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub iops: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkResource {
  #[serde(
    rename(deserialize = "mode", serialize = "Mode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub mode: Option<String>,
  #[serde(
    rename(deserialize = "device", serialize = "Device"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub device: Option<String>,
  #[serde(
    rename(deserialize = "cidr", serialize = "CIDR"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub cidr: Option<String>,
  #[serde(
    rename(deserialize = "ip", serialize = "IP"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub ip: Option<String>,
  #[serde(
    rename(deserialize = "dns", serialize = "DNS"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub dns: Option<DNSConfig>,
  #[serde(
    rename(deserialize = "reserved_ports", serialize = "ReservedPorts"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub reserved_ports: Option<Vec<Port>>,
  #[serde(
    rename(deserialize = "port", serialize = "DynamicPorts"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub port: Option<Vec<Port>>,
  #[serde(
    rename(deserialize = "hostname", serialize = "Hostname"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub hostname: Option<String>,
  #[serde(
    rename(deserialize = "mbits", serialize = "MBits"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub mbits: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DNSConfig {
  #[serde(
    rename(deserialize = "servers", serialize = "Servers"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub servers: Option<Vec<String>>,
  #[serde(
    rename(deserialize = "searches", serialize = "Searches"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub searches: Option<Vec<String>>,
  #[serde(
    rename(deserialize = "options", serialize = "Options"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub options: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Port {
  #[serde(
    rename(deserialize = "label", serialize = "Label"),
    alias = "__label__",
    default
  )]
  pub label: String,
  #[serde(
    rename(deserialize = "static", serialize = "Value"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub r#static: Option<isize>,
  #[serde(
    rename(deserialize = "to", serialize = "To"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub to: Option<isize>,
  #[serde(
    rename(deserialize = "host_network", serialize = "HostNetwork"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub host_network: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RequestedDevice {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    alias = "__label__",
    default
  )]
  pub name: String,
  #[serde(
    rename(deserialize = "count", serialize = "Count"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub count: Option<u64>,
  #[serde(
    rename(deserialize = "constraint", serialize = "Constraints"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub constraint: Option<Vec<Constraint>>,
  #[serde(
    rename(deserialize = "affinity", serialize = "Affinities"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub affinity: Option<Vec<Affinity>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LogConfig {
  #[serde(
    rename(deserialize = "max_files", serialize = "MaxFiles"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub max_files: Option<isize>,
  #[serde(
    rename(deserialize = "max_file_size", serialize = "MaxFileSizeMB"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub max_file_size: Option<isize>,
  #[serde(
    rename(deserialize = "enabled", serialize = "Enabled"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub enabled: Option<bool>,
  #[serde(
    rename(deserialize = "disabled", serialize = "Disabled"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub disabled: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestartPolicy {
  #[serde(
    rename(deserialize = "interval", serialize = "Interval"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub interval: Option<i64>,
  #[serde(
    rename(deserialize = "attempts", serialize = "Attempts"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub attempts: Option<isize>,
  #[serde(
    rename(deserialize = "delay", serialize = "Delay"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub delay: Option<i64>,
  #[serde(
    rename(deserialize = "mode", serialize = "Mode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub mode: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TaskArtifact {
  #[serde(
    rename(deserialize = "source", serialize = "GetterSource"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub source: Option<String>,
  #[serde(
    rename(deserialize = "options", serialize = "GetterOptions"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub options: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "headers", serialize = "GetterHeaders"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub headers: Option<IndexMap<String, String>>,
  #[serde(
    rename(deserialize = "mode", serialize = "GetterMode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub mode: Option<String>,
  #[serde(
    rename(deserialize = "destination", serialize = "RelativeDest"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub destination: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Vault {
  #[serde(
    rename(deserialize = "policies", serialize = "Policies"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub policies: Option<Vec<String>>,
  #[serde(
    rename(deserialize = "namespace", serialize = "Namespace"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub namespace: Option<String>,
  #[serde(
    rename(deserialize = "env", serialize = "Env"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub env: Option<bool>,
  #[serde(
    rename(deserialize = "disable_file", serialize = "DisableFile"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub disable_file: Option<bool>,
  #[serde(
    rename(deserialize = "change_mode", serialize = "ChangeMode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub change_mode: Option<String>,
  #[serde(
    rename(deserialize = "change_signal", serialize = "ChangeSignal"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub change_signal: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Template {
  #[serde(
    rename(deserialize = "source", serialize = "SourcePath"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub source: Option<String>,
  #[serde(
    rename(deserialize = "destination", serialize = "DestPath"),
    alias = "__label__",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub destination: Option<String>,
  #[serde(
    rename(deserialize = "data", serialize = "EmbeddedTmpl"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub data: Option<String>,
  #[serde(
    rename(deserialize = "change_mode", serialize = "ChangeMode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub change_mode: Option<String>,
  #[serde(
    rename(deserialize = "change_script", serialize = "ChangeScript"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub change_script: Option<ChangeScript>,
  #[serde(
    rename(deserialize = "change_signal", serialize = "ChangeSignal"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub change_signal: Option<String>,
  #[serde(
    rename(deserialize = "splay", serialize = "Splay"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub splay: Option<i64>,
  #[serde(
    rename(deserialize = "perms", serialize = "Perms"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub perms: Option<String>,
  #[serde(
    rename(deserialize = "uid", serialize = "Uid"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub uid: Option<isize>,
  #[serde(
    rename(deserialize = "gid", serialize = "Gid"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub gid: Option<isize>,
  #[serde(
    rename(deserialize = "left_delimiter", serialize = "LeftDelim"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub left_delimiter: Option<String>,
  #[serde(
    rename(deserialize = "right_delimiter", serialize = "RightDelim"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub right_delimiter: Option<String>,
  #[serde(
    rename(deserialize = "env", serialize = "Envvars"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub env: Option<bool>,
  #[serde(
    rename(deserialize = "vault_grace", serialize = "VaultGrace"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub vault_grace: Option<i64>,
  #[serde(
    rename(deserialize = "wait", serialize = "Wait"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub wait: Option<WaitConfig>,
  #[serde(
    rename(deserialize = "error_on_missing_key", serialize = "ErrMissingKey"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub error_on_missing_key: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChangeScript {
  #[serde(rename(deserialize = "command", serialize = "Command"), default)]
  pub command: String,
  #[serde(
    rename(deserialize = "args", serialize = "Args"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub args: Option<Vec<String>>,
  #[serde(
    rename(deserialize = "timeout", serialize = "Timeout"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub timeout: Option<i64>,
  #[serde(
    rename(deserialize = "fail_on_error", serialize = "FailOnError"),
    default
  )]
  pub fail_on_error: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WaitConfig {
  #[serde(rename(deserialize = "min", serialize = "Min"), default)]
  pub min: i64,
  #[serde(rename(deserialize = "max", serialize = "Max"), default)]
  pub max: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DispatchPayloadConfig {
  #[serde(
    rename(deserialize = "file", serialize = "File"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub file: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VolumeMount {
  #[serde(
    rename(deserialize = "volume", serialize = "Volume"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub volume: Option<String>,
  #[serde(
    rename(deserialize = "destination", serialize = "Destination"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub destination: Option<String>,
  #[serde(
    rename(deserialize = "read_only", serialize = "ReadOnly"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub read_only: Option<bool>,
  #[serde(
    rename(deserialize = "propagation_mode", serialize = "PropagationMode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub propagation_mode: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TaskCSIPluginConfig {
  #[serde(
    rename(deserialize = "id", serialize = "ID"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub id: Option<String>,
  #[serde(
    rename(deserialize = "type", serialize = "Type"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub r#type: Option<String>,
  #[serde(
    rename(deserialize = "mount_dir", serialize = "MountDir"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub mount_dir: Option<String>,
  #[serde(
    rename(
      deserialize = "stage_publish_base_dir",
      serialize = "StagePublishBaseDir"
    ),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub stage_publish_base_dir: Option<String>,
  #[serde(
    rename(deserialize = "health_timeout", serialize = "HealthTimeout"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub health_timeout: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScalingPolicy {
  #[serde(
    rename(deserialize = "min", serialize = "Min"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub min: Option<i64>,
  #[serde(
    rename(deserialize = "max", serialize = "Max"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub max: Option<i64>,
  #[serde(
    rename(deserialize = "policy", serialize = "Policy"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub policy: Option<IndexMap<String, serde_json::Value>>,
  #[serde(
    rename(deserialize = "enabled", serialize = "Enabled"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub enabled: Option<bool>,
  #[serde(
    rename(deserialize = "type", serialize = "Type"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkloadIdentity {
  #[serde(
    rename(deserialize = "env", serialize = "Env"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub env: Option<bool>,
  #[serde(
    rename(deserialize = "file", serialize = "File"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub file: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Spread {
  #[serde(
    rename(deserialize = "attribute", serialize = "Attribute"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub attribute: Option<String>,
  #[serde(
    rename(deserialize = "weight", serialize = "Weight"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub weight: Option<i8>,
  #[serde(
    rename(deserialize = "target", serialize = "SpreadTarget"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub target: Option<Vec<SpreadTarget>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpreadTarget {
  #[serde(
    rename(deserialize = "value", serialize = "Value"),
    alias = "__label__",
    default
  )]
  pub value: String,
  #[serde(
    rename(deserialize = "percent", serialize = "Percent"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub percent: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VolumeRequest {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    alias = "__label__",
    default
  )]
  pub name: String,
  #[serde(
    rename(deserialize = "type", serialize = "Type"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub r#type: Option<String>,
  #[serde(
    rename(deserialize = "source", serialize = "Source"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub source: Option<String>,
  #[serde(
    rename(deserialize = "read_only", serialize = "ReadOnly"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub read_only: Option<bool>,
  #[serde(
    rename(deserialize = "access_mode", serialize = "AccessMode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub access_mode: Option<String>,
  #[serde(
    rename(deserialize = "attachment_mode", serialize = "AttachmentMode"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub attachment_mode: Option<String>,
  #[serde(
    rename(deserialize = "mount_options", serialize = "MountOptions"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub mount_options: Option<CSIMountOptions>,
  #[serde(
    rename(deserialize = "per_alloc", serialize = "PerAlloc"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub per_alloc: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CSIMountOptions {
  #[serde(
    rename(deserialize = "fs_type", serialize = "FSType"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub fs_type: Option<String>,
  #[serde(
    rename(deserialize = "mount_flags", serialize = "MountFlags"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub mount_flags: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReschedulePolicy {
  #[serde(
    rename(deserialize = "attempts", serialize = "Attempts"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub attempts: Option<isize>,
  #[serde(
    rename(deserialize = "interval", serialize = "Interval"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub interval: Option<i64>,
  #[serde(
    rename(deserialize = "delay", serialize = "Delay"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub delay: Option<i64>,
  #[serde(
    rename(deserialize = "delay_function", serialize = "DelayFunction"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub delay_function: Option<String>,
  #[serde(
    rename(deserialize = "max_delay", serialize = "MaxDelay"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub max_delay: Option<i64>,
  #[serde(
    rename(deserialize = "unlimited", serialize = "Unlimited"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub unlimited: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EphemeralDisk {
  #[serde(
    rename(deserialize = "sticky", serialize = "Sticky"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub sticky: Option<bool>,
  #[serde(
    rename(deserialize = "migrate", serialize = "Migrate"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub migrate: Option<bool>,
  #[serde(
    rename(deserialize = "size", serialize = "SizeMB"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub size: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateStrategy {
  #[serde(
    rename(deserialize = "stagger", serialize = "Stagger"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub stagger: Option<i64>,
  #[serde(
    rename(deserialize = "max_parallel", serialize = "MaxParallel"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub max_parallel: Option<isize>,
  #[serde(
    rename(deserialize = "health_check", serialize = "HealthCheck"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub health_check: Option<String>,
  #[serde(
    rename(deserialize = "min_healthy_time", serialize = "MinHealthyTime"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub min_healthy_time: Option<i64>,
  #[serde(
    rename(deserialize = "healthy_deadline", serialize = "HealthyDeadline"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub healthy_deadline: Option<i64>,
  #[serde(
    rename(deserialize = "progress_deadline", serialize = "ProgressDeadline"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub progress_deadline: Option<i64>,
  #[serde(
    rename(deserialize = "canary", serialize = "Canary"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub canary: Option<isize>,
  #[serde(
    rename(deserialize = "auto_revert", serialize = "AutoRevert"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub auto_revert: Option<bool>,
  #[serde(
    rename(deserialize = "auto_promote", serialize = "AutoPromote"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub auto_promote: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MigrateStrategy {
  #[serde(
    rename(deserialize = "max_parallel", serialize = "MaxParallel"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub max_parallel: Option<isize>,
  #[serde(
    rename(deserialize = "health_check", serialize = "HealthCheck"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub health_check: Option<String>,
  #[serde(
    rename(deserialize = "min_healthy_time", serialize = "MinHealthyTime"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub min_healthy_time: Option<i64>,
  #[serde(
    rename(deserialize = "healthy_deadline", serialize = "HealthyDeadline"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub healthy_deadline: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Consul {
  #[serde(
    rename(deserialize = "namespace", serialize = "Namespace"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub namespace: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Multiregion {
  #[serde(
    rename(deserialize = "strategy", serialize = "Strategy"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub strategy: Option<MultiregionStrategy>,
  #[serde(
    rename(deserialize = "region", serialize = "Regions"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub region: Option<Vec<MultiregionRegion>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MultiregionStrategy {
  #[serde(
    rename(deserialize = "max_parallel", serialize = "MaxParallel"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub max_parallel: Option<isize>,
  #[serde(
    rename(deserialize = "on_failure", serialize = "OnFailure"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub on_failure: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MultiregionRegion {
  #[serde(
    rename(deserialize = "name", serialize = "Name"),
    alias = "__label__",
    default
  )]
  pub name: String,
  #[serde(
    rename(deserialize = "count", serialize = "Count"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub count: Option<isize>,
  #[serde(
    rename(deserialize = "datacenters", serialize = "Datacenters"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub datacenters: Option<Vec<String>>,
  #[serde(
    rename(deserialize = "node_pool", serialize = "NodePool"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub node_pool: Option<String>,
  #[serde(
    rename(deserialize = "meta", serialize = "Meta"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub meta: Option<IndexMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodicConfig {
  #[serde(
    rename(deserialize = "enabled", serialize = "Enabled"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub enabled: Option<bool>,
  #[serde(
    rename(deserialize = "cron", serialize = "Spec"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub cron: Option<String>,
  #[serde(
    rename(deserialize = "prohibit_overlap", serialize = "ProhibitOverlap"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub prohibit_overlap: Option<bool>,
  #[serde(
    rename(deserialize = "time_zone", serialize = "TimeZone"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub time_zone: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParameterizedJobConfig {
  #[serde(
    rename(deserialize = "payload", serialize = "Payload"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub payload: Option<String>,
  #[serde(
    rename(deserialize = "meta_required", serialize = "MetaRequired"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub meta_required: Option<Vec<String>>,
  #[serde(
    rename(deserialize = "meta_optional", serialize = "MetaOptional"),
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub meta_optional: Option<Vec<String>>,
}
