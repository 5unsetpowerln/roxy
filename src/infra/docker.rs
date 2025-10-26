use crate::domain::repo::{ContainerId, ContainerInfo, EnvSpec, Runtime};

pub struct DockerForContainerRuntime {}

impl DockerForContainerRuntime {
    pub fn new() -> Self {
        todo!()
    }
}

impl Runtime for DockerForContainerRuntime {
    fn provision_and_start(&self, env_spec: &EnvSpec) -> ContainerInfo {
        todo!()
    }

    fn enter(&self) {
        todo!()
    }

    fn kill(&self) {
        todo!()
    }

    fn is_running(&self) {
        todo!()
    }
}
