use crate::domain::repo::{ContainerId, ContainerInfo, EnvSpec, Runtime};

pub struct DockerForContainerRuntime {}

impl DockerForContainerRuntime {
    pub fn new() -> Self {
        todo!()
    }
}

impl Runtime for DockerForContainerRuntime {
    fn provision_and_start(&mut self, env_spec: &EnvSpec) -> ContainerInfo {
        todo!()
    }

    fn enter(&mut self, info: &ContainerInfo) {
        todo!()
    }

    fn kill(&mut self, info: &ContainerInfo) {
        todo!()
    }

    fn is_running(&mut self) {
        todo!()
    }
}
