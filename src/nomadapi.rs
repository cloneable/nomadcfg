use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Job {
    #[serde(rename(deserialize = "region", serialize = "Region"), default)]
    pub region: Option<String>,

    #[serde(rename(deserialize = "namespace", serialize = "Namespace"), default)]
    pub namespace: Option<String>,

    #[serde(rename(deserialize = "id", serialize = "ID"), default)]
    pub id: Option<String>,

    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: Option<String>,

    #[serde(rename(deserialize = "type", serialize = "Type"), default)]
    pub r#type: Option<String>,

    #[serde(rename(deserialize = "priority", serialize = "Priority"), default)]
    pub priority: Option<isize>,

    #[serde(rename(deserialize = "all_at_once", serialize = "AllAtOnce"), default)]
    pub all_at_once: Option<bool>,

    #[serde(
        rename(deserialize = "datacenters", serialize = "Datacenters"),
        default
    )]
    pub datacenters: Option<Vec<String>>,

    #[serde(rename(deserialize = "constraint", serialize = "Constraints"), default)]
    pub constraint: Option<Vec<Constraint>>,

    #[serde(rename(deserialize = "affinity", serialize = "Affinities"), default)]
    pub affinity: Option<Vec<Affinity>>,

    #[serde(rename(deserialize = "group", serialize = "TaskGroups"), default)]
    pub group: Option<Vec<TaskGroup>>,

    #[serde(rename(deserialize = "update", serialize = "Update"), default)]
    pub update: Option<UpdateStrategy>,

    #[serde(
        rename(deserialize = "multiregion", serialize = "Multiregion"),
        default
    )]
    pub multiregion: Option<Multiregion>,

    #[serde(rename(deserialize = "spread", serialize = "Spreads"), default)]
    pub spread: Option<Vec<Spread>>,

    #[serde(rename(deserialize = "periodic", serialize = "Periodic"), default)]
    pub periodic: Option<PeriodicConfig>,

    #[serde(
        rename(deserialize = "parameterized", serialize = "ParameterizedJob"),
        default
    )]
    pub parameterized: Option<ParameterizedJobConfig>,

    #[serde(rename(deserialize = "reschedule", serialize = "Reschedule"), default)]
    pub reschedule: Option<ReschedulePolicy>,

    #[serde(rename(deserialize = "migrate", serialize = "Migrate"), default)]
    pub migrate: Option<MigrateStrategy>,

    #[serde(rename(deserialize = "meta", serialize = "Meta"), default)]
    pub meta: Option<IndexMap<String, String>>,

    #[serde(
        rename(deserialize = "consul_token", serialize = "ConsulToken"),
        default
    )]
    pub consul_token: Option<String>,

    #[serde(rename(deserialize = "vault_token", serialize = "VaultToken"), default)]
    pub vault_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Constraint {
    #[serde(rename(deserialize = "attribute", serialize = "LTarget"), default)]
    pub attribute: Option<String>,

    #[serde(rename(deserialize = "value", serialize = "RTarget"), default)]
    pub value: Option<String>,

    #[serde(rename(deserialize = "operator", serialize = "Operand"), default)]
    pub operator: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Affinity {
    #[serde(rename(deserialize = "attribute", serialize = "LTarget"), default)]
    pub attribute: Option<String>,

    #[serde(rename(deserialize = "value", serialize = "RTarget"), default)]
    pub value: Option<String>,

    #[serde(rename(deserialize = "operator", serialize = "Operand"), default)]
    pub operator: Option<String>,

    #[serde(rename(deserialize = "weight", serialize = "Weight"), default)]
    pub weight: Option<i8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskGroup {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: String,

    #[serde(rename(deserialize = "count", serialize = "Count"), default)]
    pub count: Option<isize>,

    #[serde(rename(deserialize = "constraint", serialize = "Constraints"), default)]
    pub constraint: Option<Vec<Constraint>>,

    #[serde(rename(deserialize = "affinity", serialize = "Affinities"), default)]
    pub affinity: Option<Vec<Affinity>>,

    #[serde(rename(deserialize = "task", serialize = "Tasks"), default)]
    pub task: Option<Vec<Task>>,

    #[serde(rename(deserialize = "spread", serialize = "Spreads"), default)]
    pub spread: Option<Vec<Spread>>,

    #[serde(rename(deserialize = "volume", serialize = "Volumes"), default)]
    pub volume: Option<IndexMap<String, VolumeRequest>>,

    #[serde(rename(deserialize = "restart", serialize = "RestartPolicy"), default)]
    pub restart: Option<RestartPolicy>,

    #[serde(
        rename(deserialize = "reschedule", serialize = "ReschedulePolicy"),
        default
    )]
    pub reschedule: Option<ReschedulePolicy>,

    #[serde(
        rename(deserialize = "ephemeral_disk", serialize = "EphemeralDisk"),
        default
    )]
    pub ephemeral_disk: Option<EphemeralDisk>,

    #[serde(rename(deserialize = "update", serialize = "Update"), default)]
    pub update: Option<UpdateStrategy>,

    #[serde(rename(deserialize = "migrate", serialize = "Migrate"), default)]
    pub migrate: Option<MigrateStrategy>,

    #[serde(rename(deserialize = "network", serialize = "Networks"), default)]
    pub network: Option<Vec<NetworkResource>>,

    #[serde(rename(deserialize = "meta", serialize = "Meta"), default)]
    pub meta: Option<IndexMap<String, String>>,

    #[serde(rename(deserialize = "service", serialize = "Services"), default)]
    pub service: Option<Vec<Service>>,

    #[serde(
        rename(deserialize = "shutdown_delay", serialize = "ShutdownDelay"),
        default
    )]
    pub shutdown_delay: Option<i64>,

    #[serde(
        rename(
            deserialize = "stop_after_client_disconnect",
            serialize = "StopAfterClientDisconnect"
        ),
        default
    )]
    pub stop_after_client_disconnect: Option<i64>,

    #[serde(
        rename(
            deserialize = "max_client_disconnect",
            serialize = "MaxClientDisconnect"
        ),
        default
    )]
    pub max_client_disconnect: Option<i64>,

    #[serde(rename(deserialize = "scaling", serialize = "Scaling"), default)]
    pub scaling: Option<ScalingPolicy>,

    #[serde(rename(deserialize = "consul", serialize = "Consul"), default)]
    pub consul: Option<Consul>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: String,

    #[serde(rename(deserialize = "driver", serialize = "Driver"), default)]
    pub driver: Option<String>,

    #[serde(rename(deserialize = "user", serialize = "User"), default)]
    pub user: Option<String>,

    #[serde(rename(deserialize = "lifecycle", serialize = "Lifecycle"), default)]
    pub lifecycle: Option<TaskLifecycle>,

    #[serde(rename(deserialize = "config", serialize = "Config"), default)]
    pub config: Option<IndexMap<String, serde_json::Value>>,

    #[serde(rename(deserialize = "constraint", serialize = "Constraints"), default)]
    pub constraint: Option<Vec<Constraint>>,

    #[serde(rename(deserialize = "affinity", serialize = "Affinities"), default)]
    pub affinity: Option<Vec<Affinity>>,

    #[serde(rename(deserialize = "env", serialize = "Env"), default)]
    pub env: Option<IndexMap<String, String>>,

    #[serde(rename(deserialize = "service", serialize = "Services"), default)]
    pub service: Option<Vec<Service>>,

    #[serde(rename(deserialize = "resources", serialize = "Resources"), default)]
    pub resources: Option<Resources>,

    #[serde(rename(deserialize = "restart", serialize = "RestartPolicy"), default)]
    pub restart: Option<RestartPolicy>,

    #[serde(rename(deserialize = "meta", serialize = "Meta"), default)]
    pub meta: Option<IndexMap<String, String>>,

    #[serde(
        rename(deserialize = "kill_timeout", serialize = "KillTimeout"),
        default
    )]
    pub kill_timeout: Option<i64>,

    #[serde(rename(deserialize = "logs", serialize = "LogConfig"), default)]
    pub logs: Option<LogConfig>,

    #[serde(rename(deserialize = "artifact", serialize = "Artifacts"), default)]
    pub artifact: Option<Vec<TaskArtifact>>,

    #[serde(rename(deserialize = "vault", serialize = "Vault"), default)]
    pub vault: Option<Vault>,

    #[serde(rename(deserialize = "template", serialize = "Templates"), default)]
    pub template: Option<Vec<Template>>,

    #[serde(
        rename(deserialize = "dispatch_payload", serialize = "DispatchPayload"),
        default
    )]
    pub dispatch_payload: Option<DispatchPayloadConfig>,

    #[serde(
        rename(deserialize = "volume_mount", serialize = "VolumeMounts"),
        default
    )]
    pub volume_mount: Option<Vec<VolumeMount>>,

    #[serde(
        rename(deserialize = "csi_plugin", serialize = "CSIPluginConfig"),
        default
    )]
    pub csi_plugin: Option<TaskCSIPluginConfig>,

    #[serde(rename(deserialize = "leader", serialize = "Leader"), default)]
    pub leader: Option<bool>,

    #[serde(
        rename(deserialize = "shutdown_delay", serialize = "ShutdownDelay"),
        default
    )]
    pub shutdown_delay: Option<i64>,

    #[serde(rename(deserialize = "kill_signal", serialize = "KillSignal"), default)]
    pub kill_signal: Option<String>,

    #[serde(rename(deserialize = "kind", serialize = "Kind"), default)]
    pub kind: Option<String>,

    #[serde(
        rename(deserialize = "scaling", serialize = "ScalingPolicies"),
        default
    )]
    pub scaling: Option<Vec<ScalingPolicy>>,

    #[serde(rename(deserialize = "identity", serialize = "Identity"), default)]
    pub identity: Option<WorkloadIdentity>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskLifecycle {
    #[serde(rename(deserialize = "hook", serialize = "Hook"), default)]
    pub hook: Option<String>,

    #[serde(rename(deserialize = "sidecar", serialize = "Sidecar"), default)]
    pub sidecar: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Service {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: Option<String>,

    #[serde(rename(deserialize = "tags", serialize = "Tags"), default)]
    pub tags: Option<Vec<String>>,

    #[serde(rename(deserialize = "canary_tags", serialize = "CanaryTags"), default)]
    pub canary_tags: Option<Vec<String>>,

    #[serde(
        rename(deserialize = "enable_tag_override", serialize = "EnableTagOverride"),
        default
    )]
    pub enable_tag_override: Option<bool>,

    #[serde(rename(deserialize = "port", serialize = "PortLabel"), default)]
    pub port: Option<String>,

    #[serde(
        rename(deserialize = "address_mode", serialize = "AddressMode"),
        default
    )]
    pub address_mode: Option<String>,

    #[serde(rename(deserialize = "address", serialize = "Address"), default)]
    pub address: Option<String>,

    #[serde(rename(deserialize = "check", serialize = "Checks"), default)]
    pub check: Option<Vec<ServiceCheck>>,

    #[serde(
        rename(deserialize = "check_restart", serialize = "CheckRestart"),
        default
    )]
    pub check_restart: Option<CheckRestart>,

    #[serde(rename(deserialize = "connect", serialize = "Connect"), default)]
    pub connect: Option<ConsulConnect>,

    #[serde(rename(deserialize = "meta", serialize = "Meta"), default)]
    pub meta: Option<IndexMap<String, String>>,

    #[serde(rename(deserialize = "canary_meta", serialize = "CanaryMeta"), default)]
    pub canary_meta: Option<IndexMap<String, String>>,

    #[serde(
        rename(deserialize = "tagged_addresses", serialize = "TaggedAddresses"),
        default
    )]
    pub tagged_addresses: Option<IndexMap<String, String>>,

    #[serde(rename(deserialize = "task", serialize = "TaskName"), default)]
    pub task: Option<String>,

    #[serde(rename(deserialize = "on_update", serialize = "OnUpdate"), default)]
    pub on_update: Option<String>,

    #[serde(rename(deserialize = "provider", serialize = "Provider"), default)]
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceCheck {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: Option<String>,

    #[serde(rename(deserialize = "type", serialize = "Type"), default)]
    pub r#type: Option<String>,

    #[serde(rename(deserialize = "command", serialize = "Command"), default)]
    pub command: Option<String>,

    #[serde(rename(deserialize = "args", serialize = "Args"), default)]
    pub args: Option<Vec<String>>,

    #[serde(rename(deserialize = "path", serialize = "Path"), default)]
    pub path: Option<String>,

    #[serde(rename(deserialize = "protocol", serialize = "Protocol"), default)]
    pub protocol: Option<String>,

    #[serde(rename(deserialize = "port", serialize = "PortLabel"), default)]
    pub port: Option<String>,

    #[serde(rename(deserialize = "expose", serialize = "Expose"), default)]
    pub expose: Option<bool>,

    #[serde(
        rename(deserialize = "address_mode", serialize = "AddressMode"),
        default
    )]
    pub address_mode: Option<String>,

    #[serde(rename(deserialize = "advertise", serialize = "Advertise"), default)]
    pub advertise: Option<String>,

    #[serde(rename(deserialize = "interval", serialize = "Interval"), default)]
    pub interval: Option<i64>,

    #[serde(rename(deserialize = "timeout", serialize = "Timeout"), default)]
    pub timeout: Option<i64>,

    #[serde(
        rename(deserialize = "initial_status", serialize = "InitialStatus"),
        default
    )]
    pub initial_status: Option<String>,

    #[serde(
        rename(deserialize = "tls_skip_verify", serialize = "TLSSkipVerify"),
        default
    )]
    pub tls_skip_verify: Option<bool>,

    #[serde(rename(deserialize = "header", serialize = "Header"), default)]
    pub header: Option<IndexMap<String, Vec<String>>>,

    #[serde(rename(deserialize = "method", serialize = "Method"), default)]
    pub method: Option<String>,

    #[serde(
        rename(deserialize = "check_restart", serialize = "CheckRestart"),
        default
    )]
    pub check_restart: Option<CheckRestart>,

    #[serde(
        rename(deserialize = "grpc_service", serialize = "GRPCService"),
        default
    )]
    pub grpc_service: Option<String>,

    #[serde(
        rename(deserialize = "grpc_use_tls", serialize = "GRPCUseTLS"),
        default
    )]
    pub grpc_use_tls: Option<bool>,

    #[serde(rename(deserialize = "task", serialize = "TaskName"), default)]
    pub task: Option<String>,

    #[serde(
        rename(
            deserialize = "success_before_passing",
            serialize = "SuccessBeforePassing"
        ),
        default
    )]
    pub success_before_passing: Option<isize>,

    #[serde(
        rename(
            deserialize = "failures_before_critical",
            serialize = "FailuresBeforeCritical"
        ),
        default
    )]
    pub failures_before_critical: Option<isize>,

    #[serde(rename(deserialize = "body", serialize = "Body"), default)]
    pub body: Option<String>,

    #[serde(rename(deserialize = "on_update", serialize = "OnUpdate"), default)]
    pub on_update: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckRestart {
    #[serde(rename(deserialize = "limit", serialize = "Limit"), default)]
    pub limit: Option<isize>,

    #[serde(rename(deserialize = "grace", serialize = "Grace"), default)]
    pub grace: Option<i64>,

    #[serde(
        rename(deserialize = "ignore_warnings", serialize = "IgnoreWarnings"),
        default
    )]
    pub ignore_warnings: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulConnect {
    #[serde(rename(deserialize = "native", serialize = "Native"), default)]
    pub native: Option<bool>,

    #[serde(rename(deserialize = "gateway", serialize = "Gateway"), default)]
    pub gateway: Option<ConsulGateway>,

    #[serde(
        rename(deserialize = "sidecar_service", serialize = "SidecarService"),
        default
    )]
    pub sidecar_service: Option<ConsulSidecarService>,

    #[serde(
        rename(deserialize = "sidecar_task", serialize = "SidecarTask"),
        default
    )]
    pub sidecar_task: Option<SidecarTask>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulGateway {
    #[serde(rename(deserialize = "proxy", serialize = "Proxy"), default)]
    pub proxy: Option<ConsulGatewayProxy>,

    #[serde(rename(deserialize = "ingress", serialize = "Ingress"), default)]
    pub ingress: Option<ConsulIngressConfigEntry>,

    #[serde(
        rename(deserialize = "terminating", serialize = "Terminating"),
        default
    )]
    pub terminating: Option<ConsulTerminatingConfigEntry>,

    #[serde(rename(deserialize = "mesh", serialize = "Mesh"), default)]
    pub mesh: Option<ConsulMeshConfigEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulGatewayProxy {
    #[serde(
        rename(deserialize = "connect_timeout", serialize = "ConnectTimeout"),
        default
    )]
    pub connect_timeout: Option<i64>,

    #[serde(
        rename(
            deserialize = "envoy_gateway_bind_tagged_addresses",
            serialize = "EnvoyGatewayBindTaggedAddresses"
        ),
        default
    )]
    pub envoy_gateway_bind_tagged_addresses: Option<bool>,

    #[serde(
        rename(
            deserialize = "envoy_gateway_bind_addresses",
            serialize = "EnvoyGatewayBindAddresses"
        ),
        default
    )]
    pub envoy_gateway_bind_addresses: Option<IndexMap<String, ConsulGatewayBindAddress>>,

    #[serde(
        rename(
            deserialize = "envoy_gateway_no_default_bind",
            serialize = "EnvoyGatewayNoDefaultBind"
        ),
        default
    )]
    pub envoy_gateway_no_default_bind: Option<bool>,

    #[serde(
        rename(
            deserialize = "envoy_dns_discovery_type",
            serialize = "EnvoyDNSDiscoveryType"
        ),
        default
    )]
    pub envoy_dns_discovery_type: Option<String>,

    #[serde(rename(deserialize = "config", serialize = "Config"), default)]
    pub config: Option<IndexMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulGatewayBindAddress {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: String,

    #[serde(rename(deserialize = "address", serialize = "Address"), default)]
    pub address: Option<String>,

    #[serde(rename(deserialize = "port", serialize = "Port"), default)]
    pub port: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulIngressConfigEntry {
    #[serde(rename(deserialize = "tls", serialize = "TLS"), default)]
    pub tls: Option<ConsulGatewayTLSConfig>,

    #[serde(rename(deserialize = "listener", serialize = "Listeners"), default)]
    pub listener: Option<Vec<ConsulIngressListener>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulGatewayTLSConfig {
    #[serde(rename(deserialize = "enabled", serialize = "Enabled"), default)]
    pub enabled: Option<bool>,

    #[serde(
        rename(deserialize = "tls_min_version", serialize = "TLSMinVersion"),
        default
    )]
    pub tls_min_version: Option<String>,

    #[serde(
        rename(deserialize = "tls_max_version", serialize = "TLSMaxVersion"),
        default
    )]
    pub tls_max_version: Option<String>,

    #[serde(
        rename(deserialize = "cipher_suites", serialize = "CipherSuites"),
        default
    )]
    pub cipher_suites: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulIngressListener {
    #[serde(rename(deserialize = "port", serialize = "Port"), default)]
    pub port: Option<isize>,

    #[serde(rename(deserialize = "protocol", serialize = "Protocol"), default)]
    pub protocol: Option<String>,

    #[serde(rename(deserialize = "service", serialize = "Services"), default)]
    pub service: Option<Vec<ConsulIngressService>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulIngressService {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: Option<String>,

    #[serde(rename(deserialize = "hosts", serialize = "Hosts"), default)]
    pub hosts: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulTerminatingConfigEntry {
    #[serde(rename(deserialize = "service", serialize = "Services"), default)]
    pub service: Option<Vec<ConsulLinkedService>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulLinkedService {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: Option<String>,

    #[serde(rename(deserialize = "ca_file", serialize = "CAFile"), default)]
    pub ca_file: Option<String>,

    #[serde(rename(deserialize = "cert_file", serialize = "CertFile"), default)]
    pub cert_file: Option<String>,

    #[serde(rename(deserialize = "key_file", serialize = "KeyFile"), default)]
    pub key_file: Option<String>,

    #[serde(rename(deserialize = "sni", serialize = "SNI"), default)]
    pub sni: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulMeshConfigEntry {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulSidecarService {
    #[serde(rename(deserialize = "tags", serialize = "Tags"), default)]
    pub tags: Option<Vec<String>>,

    #[serde(rename(deserialize = "port", serialize = "Port"), default)]
    pub port: Option<String>,

    #[serde(rename(deserialize = "proxy", serialize = "Proxy"), default)]
    pub proxy: Option<ConsulProxy>,

    #[serde(
        rename(
            deserialize = "disable_default_tcp_check",
            serialize = "DisableDefaultTCPCheck"
        ),
        default
    )]
    pub disable_default_tcp_check: Option<bool>,

    #[serde(rename(deserialize = "meta", serialize = "Meta"), default)]
    pub meta: Option<IndexMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulProxy {
    #[serde(
        rename(
            deserialize = "local_service_address",
            serialize = "LocalServiceAddress"
        ),
        default
    )]
    pub local_service_address: Option<String>,

    #[serde(
        rename(deserialize = "local_service_port", serialize = "LocalServicePort"),
        default
    )]
    pub local_service_port: Option<isize>,

    #[serde(rename(deserialize = "expose", serialize = "Expose"), default)]
    pub expose: Option<ConsulExposeConfig>,

    #[serde(rename(deserialize = "upstreams", serialize = "Upstreams"), default)]
    pub upstreams: Option<Vec<ConsulUpstream>>,

    #[serde(rename(deserialize = "config", serialize = "Config"), default)]
    pub config: Option<IndexMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulExposeConfig {
    #[serde(rename(deserialize = "path", serialize = "Paths"), default)]
    pub path: Option<Vec<ConsulExposePath>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulExposePath {
    #[serde(rename(deserialize = "path", serialize = "Path"), default)]
    pub path: Option<String>,

    #[serde(rename(deserialize = "protocol", serialize = "Protocol"), default)]
    pub protocol: Option<String>,

    #[serde(
        rename(deserialize = "local_path_port", serialize = "LocalPathPort"),
        default
    )]
    pub local_path_port: Option<isize>,

    #[serde(
        rename(deserialize = "listener_port", serialize = "ListenerPort"),
        default
    )]
    pub listener_port: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulUpstream {
    #[serde(
        rename(deserialize = "destination_name", serialize = "DestinationName"),
        default
    )]
    pub destination_name: Option<String>,

    #[serde(
        rename(
            deserialize = "destination_namespace",
            serialize = "DestinationNamespace"
        ),
        default
    )]
    pub destination_namespace: Option<String>,

    #[serde(
        rename(deserialize = "local_bind_port", serialize = "LocalBindPort"),
        default
    )]
    pub local_bind_port: Option<isize>,

    #[serde(rename(deserialize = "datacenter", serialize = "Datacenter"), default)]
    pub datacenter: Option<String>,

    #[serde(
        rename(deserialize = "local_bind_address", serialize = "LocalBindAddress"),
        default
    )]
    pub local_bind_address: Option<String>,

    #[serde(
        rename(deserialize = "mesh_gateway", serialize = "MeshGateway"),
        default
    )]
    pub mesh_gateway: Option<ConsulMeshGateway>,

    #[serde(rename(deserialize = "config", serialize = "Config"), default)]
    pub config: Option<IndexMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConsulMeshGateway {
    #[serde(rename(deserialize = "mode", serialize = "Mode"), default)]
    pub mode: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SidecarTask {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: Option<String>,

    #[serde(rename(deserialize = "driver", serialize = "Driver"), default)]
    pub driver: Option<String>,

    #[serde(rename(deserialize = "user", serialize = "User"), default)]
    pub user: Option<String>,

    #[serde(rename(deserialize = "config", serialize = "Config"), default)]
    pub config: Option<IndexMap<String, serde_json::Value>>,

    #[serde(rename(deserialize = "env", serialize = "Env"), default)]
    pub env: Option<IndexMap<String, String>>,

    #[serde(rename(deserialize = "resources", serialize = "Resources"), default)]
    pub resources: Option<Resources>,

    #[serde(rename(deserialize = "meta", serialize = "Meta"), default)]
    pub meta: Option<IndexMap<String, String>>,

    #[serde(
        rename(deserialize = "kill_timeout", serialize = "KillTimeout"),
        default
    )]
    pub kill_timeout: Option<i64>,

    #[serde(rename(deserialize = "logs", serialize = "LogConfig"), default)]
    pub logs: Option<LogConfig>,

    #[serde(
        rename(deserialize = "shutdown_delay", serialize = "ShutdownDelay"),
        default
    )]
    pub shutdown_delay: Option<i64>,

    #[serde(rename(deserialize = "kill_signal", serialize = "KillSignal"), default)]
    pub kill_signal: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Resources {
    #[serde(rename(deserialize = "cpu", serialize = "CPU"), default)]
    pub cpu: Option<isize>,

    #[serde(rename(deserialize = "cores", serialize = "Cores"), default)]
    pub cores: Option<isize>,

    #[serde(rename(deserialize = "memory", serialize = "MemoryMB"), default)]
    pub memory: Option<isize>,

    #[serde(rename(deserialize = "memory_max", serialize = "MemoryMaxMB"), default)]
    pub memory_max: Option<isize>,

    #[serde(rename(deserialize = "disk", serialize = "DiskMB"), default)]
    pub disk: Option<isize>,

    #[serde(rename(deserialize = "network", serialize = "Networks"), default)]
    pub network: Option<Vec<NetworkResource>>,

    #[serde(rename(deserialize = "device", serialize = "Devices"), default)]
    pub device: Option<Vec<RequestedDevice>>,

    #[serde(rename(deserialize = "iops", serialize = "IOPS"), default)]
    pub iops: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkResource {
    #[serde(rename(deserialize = "mode", serialize = "Mode"), default)]
    pub mode: Option<String>,

    #[serde(rename(deserialize = "device", serialize = "Device"), default)]
    pub device: Option<String>,

    #[serde(rename(deserialize = "cidr", serialize = "CIDR"), default)]
    pub cidr: Option<String>,

    #[serde(rename(deserialize = "ip", serialize = "IP"), default)]
    pub ip: Option<String>,

    #[serde(rename(deserialize = "dns", serialize = "DNS"), default)]
    pub dns: Option<DNSConfig>,

    #[serde(
        rename(deserialize = "reserved_ports", serialize = "ReservedPorts"),
        default
    )]
    pub reserved_ports: Option<Vec<Port>>,

    #[serde(rename(deserialize = "port", serialize = "DynamicPorts"), default)]
    pub port: Option<Vec<Port>>,

    #[serde(rename(deserialize = "hostname", serialize = "Hostname"), default)]
    pub hostname: Option<String>,

    #[serde(rename(deserialize = "mbits", serialize = "MBits"), default)]
    pub mbits: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DNSConfig {
    #[serde(rename(deserialize = "servers", serialize = "Servers"), default)]
    pub servers: Option<Vec<String>>,

    #[serde(rename(deserialize = "searches", serialize = "Searches"), default)]
    pub searches: Option<Vec<String>>,

    #[serde(rename(deserialize = "options", serialize = "Options"), default)]
    pub options: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Port {
    #[serde(rename(deserialize = "label", serialize = "Label"), default)]
    pub label: String,

    #[serde(rename(deserialize = "static", serialize = "Value"), default)]
    pub r#static: Option<isize>,

    #[serde(rename(deserialize = "to", serialize = "To"), default)]
    pub to: Option<isize>,

    #[serde(
        rename(deserialize = "host_network", serialize = "HostNetwork"),
        default
    )]
    pub host_network: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestedDevice {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: String,

    #[serde(rename(deserialize = "count", serialize = "Count"), default)]
    pub count: Option<u64>,

    #[serde(rename(deserialize = "constraint", serialize = "Constraints"), default)]
    pub constraint: Option<Vec<Constraint>>,

    #[serde(rename(deserialize = "affinity", serialize = "Affinities"), default)]
    pub affinity: Option<Vec<Affinity>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogConfig {
    #[serde(rename(deserialize = "max_files", serialize = "MaxFiles"), default)]
    pub max_files: Option<isize>,

    #[serde(
        rename(deserialize = "max_file_size", serialize = "MaxFileSizeMB"),
        default
    )]
    pub max_file_size: Option<isize>,

    #[serde(rename(deserialize = "enabled", serialize = "Enabled"), default)]
    pub enabled: Option<bool>,

    #[serde(rename(deserialize = "disabled", serialize = "Disabled"), default)]
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RestartPolicy {
    #[serde(rename(deserialize = "interval", serialize = "Interval"), default)]
    pub interval: Option<i64>,

    #[serde(rename(deserialize = "attempts", serialize = "Attempts"), default)]
    pub attempts: Option<isize>,

    #[serde(rename(deserialize = "delay", serialize = "Delay"), default)]
    pub delay: Option<i64>,

    #[serde(rename(deserialize = "mode", serialize = "Mode"), default)]
    pub mode: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskArtifact {
    #[serde(rename(deserialize = "source", serialize = "GetterSource"), default)]
    pub source: Option<String>,

    #[serde(rename(deserialize = "options", serialize = "GetterOptions"), default)]
    pub options: Option<IndexMap<String, String>>,

    #[serde(rename(deserialize = "headers", serialize = "GetterHeaders"), default)]
    pub headers: Option<IndexMap<String, String>>,

    #[serde(rename(deserialize = "mode", serialize = "GetterMode"), default)]
    pub mode: Option<String>,

    #[serde(
        rename(deserialize = "destination", serialize = "RelativeDest"),
        default
    )]
    pub destination: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vault {
    #[serde(rename(deserialize = "policies", serialize = "Policies"), default)]
    pub policies: Option<Vec<String>>,

    #[serde(rename(deserialize = "namespace", serialize = "Namespace"), default)]
    pub namespace: Option<String>,

    #[serde(rename(deserialize = "env", serialize = "Env"), default)]
    pub env: Option<bool>,

    #[serde(rename(deserialize = "change_mode", serialize = "ChangeMode"), default)]
    pub change_mode: Option<String>,

    #[serde(
        rename(deserialize = "change_signal", serialize = "ChangeSignal"),
        default
    )]
    pub change_signal: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Template {
    #[serde(rename(deserialize = "source", serialize = "SourcePath"), default)]
    pub source: Option<String>,

    #[serde(rename(deserialize = "destination", serialize = "DestPath"), default)]
    pub destination: Option<String>,

    #[serde(rename(deserialize = "data", serialize = "EmbeddedTmpl"), default)]
    pub data: Option<String>,

    #[serde(rename(deserialize = "change_mode", serialize = "ChangeMode"), default)]
    pub change_mode: Option<String>,

    #[serde(
        rename(deserialize = "change_script", serialize = "ChangeScript"),
        default
    )]
    pub change_script: Option<ChangeScript>,

    #[serde(
        rename(deserialize = "change_signal", serialize = "ChangeSignal"),
        default
    )]
    pub change_signal: Option<String>,

    #[serde(rename(deserialize = "splay", serialize = "Splay"), default)]
    pub splay: Option<i64>,

    #[serde(rename(deserialize = "perms", serialize = "Perms"), default)]
    pub perms: Option<String>,

    #[serde(rename(deserialize = "uid", serialize = "Uid"), default)]
    pub uid: Option<isize>,

    #[serde(rename(deserialize = "gid", serialize = "Gid"), default)]
    pub gid: Option<isize>,

    #[serde(
        rename(deserialize = "left_delimiter", serialize = "LeftDelim"),
        default
    )]
    pub left_delimiter: Option<String>,

    #[serde(
        rename(deserialize = "right_delimiter", serialize = "RightDelim"),
        default
    )]
    pub right_delimiter: Option<String>,

    #[serde(rename(deserialize = "env", serialize = "Envvars"), default)]
    pub env: Option<bool>,

    #[serde(rename(deserialize = "vault_grace", serialize = "VaultGrace"), default)]
    pub vault_grace: Option<i64>,

    #[serde(rename(deserialize = "wait", serialize = "Wait"), default)]
    pub wait: Option<WaitConfig>,

    #[serde(
        rename(deserialize = "error_on_missing_key", serialize = "ErrMissingKey"),
        default
    )]
    pub error_on_missing_key: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangeScript {
    #[serde(rename(deserialize = "command", serialize = "Command"), default)]
    pub command: String,

    #[serde(rename(deserialize = "args", serialize = "Args"), default)]
    pub args: Option<Vec<String>>,

    #[serde(rename(deserialize = "timeout", serialize = "Timeout"), default)]
    pub timeout: Option<i64>,

    #[serde(
        rename(deserialize = "fail_on_error", serialize = "FailOnError"),
        default
    )]
    pub fail_on_error: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WaitConfig {
    #[serde(rename(deserialize = "min", serialize = "Min"), default)]
    pub min: i64,

    #[serde(rename(deserialize = "max", serialize = "Max"), default)]
    pub max: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DispatchPayloadConfig {
    #[serde(rename(deserialize = "file", serialize = "File"), default)]
    pub file: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VolumeMount {
    #[serde(rename(deserialize = "volume", serialize = "Volume"), default)]
    pub volume: Option<String>,

    #[serde(
        rename(deserialize = "destination", serialize = "Destination"),
        default
    )]
    pub destination: Option<String>,

    #[serde(rename(deserialize = "read_only", serialize = "ReadOnly"), default)]
    pub read_only: Option<bool>,

    #[serde(
        rename(deserialize = "propagation_mode", serialize = "PropagationMode"),
        default
    )]
    pub propagation_mode: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskCSIPluginConfig {
    #[serde(rename(deserialize = "id", serialize = "ID"), default)]
    pub id: Option<String>,

    #[serde(rename(deserialize = "type", serialize = "Type"), default)]
    pub r#type: Option<String>,

    #[serde(rename(deserialize = "mount_dir", serialize = "MountDir"), default)]
    pub mount_dir: Option<String>,

    #[serde(
        rename(
            deserialize = "stage_publish_base_dir",
            serialize = "StagePublishBaseDir"
        ),
        default
    )]
    pub stage_publish_base_dir: Option<String>,

    #[serde(
        rename(deserialize = "health_timeout", serialize = "HealthTimeout"),
        default
    )]
    pub health_timeout: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalingPolicy {
    #[serde(rename(deserialize = "min", serialize = "Min"), default)]
    pub min: Option<i64>,

    #[serde(rename(deserialize = "max", serialize = "Max"), default)]
    pub max: Option<i64>,

    #[serde(rename(deserialize = "policy", serialize = "Policy"), default)]
    pub policy: Option<IndexMap<String, serde_json::Value>>,

    #[serde(rename(deserialize = "enabled", serialize = "Enabled"), default)]
    pub enabled: Option<bool>,

    #[serde(rename(deserialize = "type", serialize = "Type"), default)]
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkloadIdentity {
    #[serde(rename(deserialize = "env", serialize = "Env"), default)]
    pub env: Option<bool>,

    #[serde(rename(deserialize = "file", serialize = "File"), default)]
    pub file: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Spread {
    #[serde(rename(deserialize = "attribute", serialize = "Attribute"), default)]
    pub attribute: Option<String>,

    #[serde(rename(deserialize = "weight", serialize = "Weight"), default)]
    pub weight: Option<i8>,

    #[serde(rename(deserialize = "target", serialize = "SpreadTarget"), default)]
    pub target: Option<Vec<SpreadTarget>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpreadTarget {
    #[serde(rename(deserialize = "value", serialize = "Value"), default)]
    pub value: String,

    #[serde(rename(deserialize = "percent", serialize = "Percent"), default)]
    pub percent: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VolumeRequest {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: String,

    #[serde(rename(deserialize = "type", serialize = "Type"), default)]
    pub r#type: Option<String>,

    #[serde(rename(deserialize = "source", serialize = "Source"), default)]
    pub source: Option<String>,

    #[serde(rename(deserialize = "read_only", serialize = "ReadOnly"), default)]
    pub read_only: Option<bool>,

    #[serde(rename(deserialize = "access_mode", serialize = "AccessMode"), default)]
    pub access_mode: Option<String>,

    #[serde(
        rename(deserialize = "attachment_mode", serialize = "AttachmentMode"),
        default
    )]
    pub attachment_mode: Option<String>,

    #[serde(
        rename(deserialize = "mount_options", serialize = "MountOptions"),
        default
    )]
    pub mount_options: Option<CSIMountOptions>,

    #[serde(rename(deserialize = "per_alloc", serialize = "PerAlloc"), default)]
    pub per_alloc: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CSIMountOptions {
    #[serde(rename(deserialize = "fs_type", serialize = "FSType"), default)]
    pub fs_type: Option<String>,

    #[serde(rename(deserialize = "mount_flags", serialize = "MountFlags"), default)]
    pub mount_flags: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReschedulePolicy {
    #[serde(rename(deserialize = "attempts", serialize = "Attempts"), default)]
    pub attempts: Option<isize>,

    #[serde(rename(deserialize = "interval", serialize = "Interval"), default)]
    pub interval: Option<i64>,

    #[serde(rename(deserialize = "delay", serialize = "Delay"), default)]
    pub delay: Option<i64>,

    #[serde(
        rename(deserialize = "delay_function", serialize = "DelayFunction"),
        default
    )]
    pub delay_function: Option<String>,

    #[serde(rename(deserialize = "max_delay", serialize = "MaxDelay"), default)]
    pub max_delay: Option<i64>,

    #[serde(rename(deserialize = "unlimited", serialize = "Unlimited"), default)]
    pub unlimited: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EphemeralDisk {
    #[serde(rename(deserialize = "sticky", serialize = "Sticky"), default)]
    pub sticky: Option<bool>,

    #[serde(rename(deserialize = "migrate", serialize = "Migrate"), default)]
    pub migrate: Option<bool>,

    #[serde(rename(deserialize = "size", serialize = "SizeMB"), default)]
    pub size: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateStrategy {
    #[serde(rename(deserialize = "stagger", serialize = "Stagger"), default)]
    pub stagger: Option<i64>,

    #[serde(
        rename(deserialize = "max_parallel", serialize = "MaxParallel"),
        default
    )]
    pub max_parallel: Option<isize>,

    #[serde(
        rename(deserialize = "health_check", serialize = "HealthCheck"),
        default
    )]
    pub health_check: Option<String>,

    #[serde(
        rename(deserialize = "min_healthy_time", serialize = "MinHealthyTime"),
        default
    )]
    pub min_healthy_time: Option<i64>,

    #[serde(
        rename(deserialize = "healthy_deadline", serialize = "HealthyDeadline"),
        default
    )]
    pub healthy_deadline: Option<i64>,

    #[serde(
        rename(deserialize = "progress_deadline", serialize = "ProgressDeadline"),
        default
    )]
    pub progress_deadline: Option<i64>,

    #[serde(rename(deserialize = "canary", serialize = "Canary"), default)]
    pub canary: Option<isize>,

    #[serde(rename(deserialize = "auto_revert", serialize = "AutoRevert"), default)]
    pub auto_revert: Option<bool>,

    #[serde(
        rename(deserialize = "auto_promote", serialize = "AutoPromote"),
        default
    )]
    pub auto_promote: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MigrateStrategy {
    #[serde(
        rename(deserialize = "max_parallel", serialize = "MaxParallel"),
        default
    )]
    pub max_parallel: Option<isize>,

    #[serde(
        rename(deserialize = "health_check", serialize = "HealthCheck"),
        default
    )]
    pub health_check: Option<String>,

    #[serde(
        rename(deserialize = "min_healthy_time", serialize = "MinHealthyTime"),
        default
    )]
    pub min_healthy_time: Option<i64>,

    #[serde(
        rename(deserialize = "healthy_deadline", serialize = "HealthyDeadline"),
        default
    )]
    pub healthy_deadline: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Consul {
    #[serde(rename(deserialize = "namespace", serialize = "Namespace"), default)]
    pub namespace: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Multiregion {
    #[serde(rename(deserialize = "strategy", serialize = "Strategy"), default)]
    pub strategy: Option<MultiregionStrategy>,

    #[serde(rename(deserialize = "region", serialize = "Regions"), default)]
    pub region: Option<Vec<MultiregionRegion>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultiregionStrategy {
    #[serde(
        rename(deserialize = "max_parallel", serialize = "MaxParallel"),
        default
    )]
    pub max_parallel: Option<isize>,

    #[serde(rename(deserialize = "on_failure", serialize = "OnFailure"), default)]
    pub on_failure: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultiregionRegion {
    #[serde(rename(deserialize = "name", serialize = "Name"), default)]
    pub name: String,

    #[serde(rename(deserialize = "count", serialize = "Count"), default)]
    pub count: Option<isize>,

    #[serde(
        rename(deserialize = "datacenters", serialize = "Datacenters"),
        default
    )]
    pub datacenters: Option<Vec<String>>,

    #[serde(rename(deserialize = "meta", serialize = "Meta"), default)]
    pub meta: Option<IndexMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PeriodicConfig {
    #[serde(rename(deserialize = "enabled", serialize = "Enabled"), default)]
    pub enabled: Option<bool>,

    #[serde(rename(deserialize = "cron", serialize = "Spec"), default)]
    pub cron: Option<String>,

    #[serde(
        rename(deserialize = "prohibit_overlap", serialize = "ProhibitOverlap"),
        default
    )]
    pub prohibit_overlap: Option<bool>,

    #[serde(rename(deserialize = "time_zone", serialize = "TimeZone"), default)]
    pub time_zone: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParameterizedJobConfig {
    #[serde(rename(deserialize = "payload", serialize = "Payload"), default)]
    pub payload: Option<String>,

    #[serde(
        rename(deserialize = "meta_required", serialize = "MetaRequired"),
        default
    )]
    pub meta_required: Option<Vec<String>>,

    #[serde(
        rename(deserialize = "meta_optional", serialize = "MetaOptional"),
        default
    )]
    pub meta_optional: Option<Vec<String>>,
}
