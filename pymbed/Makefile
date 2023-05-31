VENV_PATH:= ${PWD}/venv
PYTHON_VERSION:=3.10

# External commands
setup: venv/create venv/add-hyperm venv/install_pkgs

remove:
	rm -rf $(VENV_PATH)

jupyter/run:
	/bin/bash -c "source $(VENV_PATH)/bin/activate && jupyter-lab"

new/book:
	/bin/bash -c "source $(VENV_PATH)/bin/activate && jupyter-book create HydrogenPermeation"

# Internal commands

venv/install_pkgs:
	/bin/bash -c "source $(VENV_PATH)/bin/activate && pip install -r requirements.txt"
	

venv/create:
	python3 -m venv $(VENV_PATH)

venv/add-hyperm:
	/bin/bash -c "echo ${PWD}/lib > $(VENV_PATH)/lib/python$(PYTHON_VERSION)/site-packages/hyperm.pth"

vagrant/add-key:
	ssh-add vagrant/.vagrant/machines/default/virtualbox/private_key


#venv/ln-activate:
#	ln -s $(VENV_PATH)/bin/activate start
