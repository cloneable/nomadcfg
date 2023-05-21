local nomad = import 'nomad.libsonnet';

{
  Job: nomad.Service {
    Name: 'cacheserver',
    Region: 'eu',
    Datacenters: ['dc1'],
    TaskGroups: [
      nomad.TaskGroup {
        Name: 'prod',
        Tasks: [
          nomad.Task {
            Name: 'redis',
            Driver: 'docker',
            Config: {
              image: 'redis:' + std.extVar('imagetag'),
            },
            Templates: [
              nomad.Template {
                DestPath: '/etc/config.yaml',
                EmbeddedTmpl: nomad.YamlTmpl({
                  log: {
                    level: 'INFO',
                  },
                  redis: {
                    password: '{{ secret "redis/prod/password" }}',
                  },
                }),
              },
            ],
          },
        ],
      },
    ],
  },
}
