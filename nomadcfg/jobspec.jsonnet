local nomad = import 'nomad.libsonnet';

local service_config = {
  log: {
    level: 'INFO',
  },
  redis: {
    password: '{{ secret "redis/prod/password" }}',
  },
};

{
  Job: nomad.Job('cacheserver') {
    Type: 'service',
    Name: 'cacheserver',
    Namespace: 'backend',

    Region: 'eu',
    Datacenters: ['dc1'],

    TaskGroups: [
      nomad.TaskGroup('prod') {
        Tasks: [
          nomad.Task('redis') {
            Driver: 'docker',
            Config: {
              image: 'redis:' + std.extVar('imagetag'),
            },

            Templates: [
              nomad.Template('/etc/config.yaml') {
                ChangeMode: 'noop',
                EmbeddedTmpl: nomad.YamlTmpl(service_config),
              },
            ],
          },
        ],
      },
    ],
  },
}
