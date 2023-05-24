{
  Job: {
    Affinities: [],
    AllAtOnce: null,
    Constraints: [],
    ConsulNamespace: null,
    ConsulToken: null,
    CreateIndex: null,
    Datacenters: [],
    DispatchIdempotencyToken: null,
    Dispatched: null,
    ID: null,
    JobModifyIndex: null,
    Meta: {},
    Migrate: null,
    ModifyIndex: null,
    Multiregion: null,
    Name: null,
    Namespace: null,
    NomadTokenID: null,
    ParameterizedJob: null,
    ParentID: null,
    Payload: null,
    Periodic: null,
    Priority: null,
    Region: null,
    Reschedule: null,
    Spreads: [],
    Stable: null,
    Status: null,
    StatusDescription: null,
    Stop: null,
    SubmitTime: null,
    TaskGroups: [],
    Type: null,
    Update: null,
    VaultNamespace: null,
    VaultToken: null,
    Version: null,
  },

  Service: self.Job {
    Type: 'service',
  },

  TaskGroup: {
    Affinities: [],
    Constraints: [],
    Consul: null,
    Count: null,
    EphemeralDisk: null,
    MaxClientDisconnect: null,
    Meta: {},
    Migrate: null,
    Name: null,
    Networks: [],
    ReschedulePolicy: null,
    RestartPolicy: null,
    Scaling: null,
    Services: [],
    ShutdownDelay: null,
    Spreads: [],
    StopAfterClientDisconnect: null,
    Tasks: [],
    Update: null,
    Volumes: {},
  },

  Task: {
    Affinities: [],
    Artifacts: [],
    Config: {},
    Constraints: [],
    CSIPluginConfig: null,
    DispatchPayload: null,
    Driver: null,
    Env: {},
    KillSignal: null,
    KillTimeout: null,
    Kind: null,
    Leader: null,
    Lifecycle: null,
    LogConfig: null,
    Meta: {},
    Name: null,
    Resources: null,
    RestartPolicy: null,
    ScalingPolicies: [],
    Services: [],
    ShutdownDelay: null,
    Templates: [],
    User: null,
    Vault: null,
    VolumeMounts: [],
  },

  Template: {
    ChangeMode: null,
    ChangeScript: null,
    ChangeSignal: null,
    DestPath: null,
    EmbeddedTmpl: null,
    Envvars: null,
    Gid: null,
    LeftDelim: null,
    Perms: null,
    RightDelim: null,
    SourcePath: null,
    Splay: null,
    Uid: null,
    VaultGrace: null,
    Wait: null,
  },

  YamlTmpl(data): std.manifestYamlDoc(data,
                                      indent_array_in_object=true,
                                      quote_keys=false,
                                      preserve_order=true),
}