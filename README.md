# k8s_wfx
## Double Commander Rust plugin to access kubernetes resources


### Example of kubectl command to create a pod without yaml file to test wfx plugin

```bash
kubectl run my-busybox-pod --image=busybox --restart=Never --command -- /bin/sh -c "sleep 3600"
```