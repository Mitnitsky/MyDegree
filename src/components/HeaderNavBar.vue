<template>
        <b-navbar toggleable="lg" type="dark" variant="dark">
            <img style="width: 2%; height: 2%;margin-right: 5px;" src="../assets/main_icon_white.svg" alt=""/>
            <b-navbar-brand href="#">Degree Planer</b-navbar-brand>

            <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>

            <b-collapse id="nav-collapse" is-nav>
\

                <!-- Right aligned nav items -->
                <b-navbar-nav class="ml-auto">
                    <b-nav-item-dropdown right v-if="logged">
                        <!-- Using 'button-content' slot -->
                        <template v-slot:button-content>
                            <em>User</em>
                        </template>
                        <b-dropdown-item href="#">Profile</b-dropdown-item>
                        <b-dropdown-item href="#">Sign Out</b-dropdown-item>
                    </b-nav-item-dropdown>
                    <b-nav-item-dropdown right v-else>
                        <!-- Using 'button-content' slot -->
                        <template v-slot:button-content>
                            <em>Login</em>
                        </template>
                        <b-dropdown-item href="#">Google</b-dropdown-item>
                        <b-dropdown-item href="#">Technion Email</b-dropdown-item>
                    </b-nav-item-dropdown>
                </b-navbar-nav>
            </b-collapse>
        </b-navbar>
</template>

<script>
    export default {
        props: ['logged'],
        name: "HeaderNavBar"
    }
</script>
